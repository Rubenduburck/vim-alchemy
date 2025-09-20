-- Real-time preview functionality for alchemy conversions
-- Shows live conversion results as you navigate options

local M = {}

local UI = require("alchemy.ui")
local Core = require("alchemy.core")

-- Active preview window
local preview_win = nil
local active_buffer_preview = nil

local function split_lines(text)
	if text == nil then
		return { "" }
	end

	local lines = vim.split(text, "\n", { plain = true })

	if vim.tbl_isempty(lines) then
		return { "" }
	end

	return lines
end

local function clone_region(selection)
	if not selection then
		return nil
	end

	return {
		start_line = selection.start_line,
		start_col = selection.start_col,
		end_line = selection.end_line,
		end_col = selection.end_col,
	}
end

local function compute_region(start_line, start_col, lines)
	local count = #lines
	local end_line = start_line
	local end_col = start_col

	if count == 0 then
		return {
			start_line = start_line,
			start_col = start_col,
			end_line = start_line,
			end_col = start_col,
		}
	end

	if count == 1 then
		end_col = start_col + #lines[1]
	else
		end_line = start_line + (count - 1)
		end_col = #lines[count]
	end

	return {
		start_line = start_line,
		start_col = start_col,
		end_line = end_line,
		end_col = end_col,
	}
end

local function set_text(bufnr, region, lines)
	vim.api.nvim_buf_set_text(
		bufnr,
		region.start_line,
		region.start_col,
		region.end_line,
		region.end_col,
		lines
	)
end

local function ensure_buffer_preview(selection)
	if active_buffer_preview then
		return true
	end

	local bufnr = vim.api.nvim_get_current_buf()

	if not selection or selection.start_line == nil or selection.start_col == nil or selection.end_line == nil or selection.end_col == nil then
		if not selection or not selection.text or selection.text == "" then
			return false
		end

		local cursor = vim.api.nvim_win_get_cursor(0)
		local start_line = cursor[1] - 1
		local start_col = cursor[2]
		local lines = split_lines(selection.text)
		local region = compute_region(start_line, start_col, lines)
		local ok, original_lines = pcall(vim.api.nvim_buf_get_text, bufnr, region.start_line, region.start_col, region.end_line, region.end_col, {})
		if not ok or vim.tbl_isempty(original_lines) then
			original_lines = lines
		end

		active_buffer_preview = {
			bufnr = bufnr,
			original = {
				lines = original_lines,
				region = region,
			},
		}
		return true
	end

	active_buffer_preview = {
		bufnr = bufnr,
		original = {
			lines = split_lines(selection.text),
			region = clone_region(selection),
		},
	}

	return true
end

local function apply_buffer_preview(selection, new_text)
	if not ensure_buffer_preview(selection) then
		return false
	end

	local lines = split_lines(new_text)
	local preview = active_buffer_preview
	local region = preview.current and preview.current.region or preview.original.region

	set_text(preview.bufnr, region, lines)

	preview.current = {
		lines = lines,
		region = compute_region(preview.original.region.start_line, preview.original.region.start_col, lines),
	}

	return true
end

local function revert_buffer_preview()
	if not active_buffer_preview or not active_buffer_preview.current then
		active_buffer_preview = nil
		return
	end

	local preview = active_buffer_preview
	set_text(preview.bufnr, preview.current.region, preview.original.lines)
	active_buffer_preview = nil
end

local function commit_buffer_preview()
	if not active_buffer_preview then
		return
	end
	active_buffer_preview = nil
end

-- Show a live preview of conversion
function M.show_conversion_preview(text_selection, conversion_fn, opts)
	opts = opts or {}

	M.close_preview()

	local ok, result = pcall(conversion_fn)
	if not ok or not result then
		UI.notify("Conversion failed: " .. tostring(result), vim.log.levels.ERROR)
		return
	end

	local buffer_preview_enabled = apply_buffer_preview(text_selection, result)
	local state = { committed = false, buffer_preview = buffer_preview_enabled }

	local preview_width = opts.width or math.min(70, math.max(20, vim.o.columns - 8))
	local preview_height = opts.height or math.min(12, math.max(5, vim.o.lines - 6))

	local preview_row, preview_col, preview_actual_width, preview_actual_height =
		UI.compute_window_geometry(text_selection, preview_width, preview_height)

	local preview_opts = {
		title = opts.title or " 🔄 Live Preview ",
		position = opts.position or "bottom_right",
		width = preview_actual_width,
		height = preview_actual_height,
		row = preview_row,
		col = preview_col,
		cursor_relative = false,
		selection = text_selection,
		on_pre_confirm = function()
			if state.buffer_preview then
				state.committed = true
				commit_buffer_preview()
			end
		end,
		on_confirm = function()
			if not state.buffer_preview then
				Core.replace_text(text_selection, result)
			end
			UI.notify("Conversion applied!", vim.log.levels.INFO)
		end,
		on_close = function()
			if state.buffer_preview and not state.committed then
				revert_buffer_preview()
			end
		end,
		original_syntax = opts.original_syntax,
		converted_syntax = opts.converted_syntax,
	}

	preview_win = UI.create_preview(text_selection.text, result, preview_opts)

	if not buffer_preview_enabled then
		UI.notify("Previewing in buffer unavailable; showing read-only preview.", vim.log.levels.WARN)
	end
end

-- Show multiple conversion options with live preview
function M.show_conversion_explorer(text_selection, input_encoding)
	M.close_preview()

	local options = {}

	local encodings = { 
		{ name = "hex", display = "Hexadecimal" },
		{ name = "int", display = "Integer" },
		{ name = "base64", display = "Base64" },
		{ name = "base58", display = "Base58" },
		{ name = "bin", display = "Binary" },
		{ name = "utf8", display = "UTF-8" },
		{ name = "ascii", display = "ASCII" },
		{ name = "bytes", display = "Bytes" },
	}

	for _, encoding in ipairs(encodings) do
		table.insert(options, {
			text = encoding.display,
			name = encoding.name,
			type = encoding.name,
			encoding = encoding.name,
		})
	end

	local hash_algos = { 
		{ name = "sha256", display = "SHA256" },
		{ name = "sha512", display = "SHA512" },
		{ name = "blake2", display = "Blake2" },
		{ name = "keccak256", display = "Keccak256" },
	}
	for _, algo in ipairs(hash_algos) do
		table.insert(options, {
			text = "Hash: " .. algo.display,
			name = algo.name,
			type = "hash",
			algorithm = algo.name,
		})
	end

	local state = {
		buffer_preview = false,
		committed = false,
		active_option = nil,
	}

	local function run_conversion(option)
		local ok, result
		if option.type == "hash" then
			ok, result = pcall(Core.execute_cli, { "hash", "-i", "utf8", "-a", option.algorithm, text_selection.text }, false)
		else
			if input_encoding then
				ok, result = pcall(Core.convert, text_selection.text, input_encoding, option.encoding)
			else
				ok, result = pcall(Core.classify_and_convert, text_selection.text, option.encoding)
			end
		end

		if not ok then
			return false, nil, tostring(result)
		end

		if result == nil then
			return false, nil, "Conversion produced no output"
		end

		return true, result, nil
	end

	local function preview_option(option)
		local ok, result, err = run_conversion(option)
		if not ok then
			revert_buffer_preview()
			state.buffer_preview = false
			state.active_option = nil
			return err or "Error: Could not convert"
		end

		if apply_buffer_preview(text_selection, result) then
			state.buffer_preview = true
			state.active_option = option
			return ''
		end

		state.buffer_preview = false
		state.active_option = option
		return string.format("Input:  %s\nOutput: %s", text_selection.text, result)
	end

	local selector_width_hint = math.min(45, math.max(20, math.floor(vim.o.columns * 0.4)))
	local selector_height_hint = math.min(#options + 2, math.max(6, math.floor(vim.o.lines * 0.4)))
	local selector_row, selector_col, selector_max_width, selector_max_height =
		UI.compute_window_geometry(text_selection, selector_width_hint, selector_height_hint)

	UI.create_selector(options, {
		title = "Convert",
		cursor_relative = false,
		row = selector_row,
		col = selector_col,
		max_width = selector_max_width,
		max_height = selector_max_height,
		selection = text_selection,
		on_preview = function(option)
			return preview_option(option)
		end,
		on_pre_select = function(option)
			if state.active_option ~= option then
				preview_option(option)
			end

			if state.buffer_preview then
				state.committed = true
				commit_buffer_preview()
				return true
			end

			local ok, result, err = run_conversion(option)
			if not ok or result == nil then
				UI.notify(err or "Conversion failed!", vim.log.levels.ERROR)
				return false
			end

			Core.replace_text(text_selection, result)
			return true
		end,
		on_select = function(option)
			UI.notify("Converted to " .. option.text, vim.log.levels.INFO)
		end,
		on_close = function()
			if state.buffer_preview and not state.committed then
				revert_buffer_preview()
			end
		end,
	})
end

-- Close any active preview
function M.close_preview()
	revert_buffer_preview()

	if not preview_win then
		return
	end
	if preview_win.close then
		preview_win.close()
	elseif preview_win.win and vim.api.nvim_win_is_valid(preview_win.win) then
		vim.api.nvim_win_close(preview_win.win, true)
	end
	preview_win = nil
end

-- Smart classification display with confidence bars
function M.show_classifications(text_selection, classifications)
	if not classifications or vim.tbl_isempty(classifications) then
		UI.notify("No classifications found", vim.log.levels.WARN)
		return
	end
	
	-- Process classifications
	local items = {}
	
	-- Find min and max scores to normalize confidence
	local min_score = math.huge
	local max_score = -math.huge
	for _, class in ipairs(classifications) do
		if class.encoding and class.score then
			min_score = math.min(min_score, class.score)
			max_score = math.max(max_score, class.score)
		end
	end
	
	for _, class in ipairs(classifications) do
		if class.encoding and class.score then
			-- Normalize score to 0-100 range where lower score = higher confidence
			local confidence = 100
			if max_score > min_score then
				confidence = math.max(0, math.min(100, 100 * (max_score - class.score) / (max_score - min_score)))
			end
			
			local confidence_bar = M.create_confidence_bar(confidence)
			table.insert(items, {
				name = string.format("%-10s %s", class.encoding, confidence_bar),
				type = class.encoding,
				confidence = confidence,
				score = class.score,
			})
		end
	end
	
	-- Sort by confidence (higher is better)
	table.sort(items, function(a, b)
		return a.confidence > b.confidence
	end)
	
	-- Show in a nice UI
	UI.create_selector(items, {
		title = " 🔍 Classification Results ",
		selection = text_selection,
		on_select = function(item)
			-- When a classification is selected, show conversion options with the selected input encoding
			M.show_conversion_explorer(text_selection, item.type)
		end,
	})
end

-- Create a visual confidence bar
function M.create_confidence_bar(percentage)
	-- Clamp percentage to 0-100 range
	percentage = math.max(0, math.min(100, percentage))
	
	local width = 20
	local filled = math.floor(width * percentage / 100)
	local empty = width - filled
	
	-- Use ASCII characters for better compatibility
	local bar = "["
	bar = bar .. string.rep("=", filled)
	bar = bar .. string.rep("-", empty)
	bar = bar .. "] " .. string.format("%3d%%", math.floor(percentage))
	
	return bar
end

return M
