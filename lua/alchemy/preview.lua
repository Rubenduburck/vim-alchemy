-- Real-time preview functionality for alchemy conversions
-- Shows live conversion results as you navigate options

local M = {}

local UI = require("alchemy.ui")
local Core = require("alchemy.core")

-- Active preview window
local preview_win = nil

-- Show a live preview of conversion
function M.show_conversion_preview(text_selection, conversion_fn, opts)
	opts = opts or {}
	
	-- Close any existing preview
	M.close_preview()
	
	-- Perform the conversion
	local ok, result = pcall(conversion_fn)
	if not ok then
		return
	end
	
	-- Create preview window
	preview_win = UI.create_preview(text_selection.text, result, {
		title = opts.title or " 🔄 Live Preview ",
		on_confirm = function()
			Core.replace_text(text_selection, result)
			UI.notify("Conversion applied!", vim.log.levels.INFO)
		end,
		original_syntax = opts.original_syntax,
		converted_syntax = opts.converted_syntax,
	})
end

-- Show multiple conversion options with live preview
function M.show_conversion_explorer(text_selection, input_encoding)
	-- Build conversion options
	local options = {}
	
	-- Add conversions for each classification
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
	
	-- Add hash operations
	local hash_algos = { 
		{ name = "sha256", display = "SHA256" },
		{ name = "sha512", display = "SHA512" },
		{ name = "md5", display = "MD5" },
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
	
	-- Create the selector with live preview
	UI.create_selector(options, {
		title = "Convert",
		cursor_relative = true,
		on_preview = function(option)
			-- Generate preview content
			local ok, result
			if option.type == "hash" then
				ok, result = pcall(Core.execute_cli, { "hash", "-i", "utf8", "-a", option.algorithm, text_selection.text }, false)
			else
				-- Use explicit input encoding if provided, otherwise auto-classify
				if input_encoding then
					ok, result = pcall(Core.convert, text_selection.text, input_encoding, option.encoding)
				else
					ok, result = pcall(Core.classify_and_convert, text_selection.text, option.encoding)
				end
			end
			
			if ok and result then
				return string.format("Input:  %s\nOutput: %s", text_selection.text, result)
			else
				return "Error: Could not convert"
			end
		end,
		on_select = function(option)
			-- Apply the conversion
			local ok, result
			if option.type == "hash" then
				ok, result = pcall(Core.execute_cli, { "hash", "-i", "utf8", "-a", option.algorithm, text_selection.text }, false)
			else
				-- Use explicit input encoding if provided, otherwise auto-classify
				if input_encoding then
					ok, result = pcall(Core.convert, text_selection.text, input_encoding, option.encoding)
				else
					ok, result = pcall(Core.classify_and_convert, text_selection.text, option.encoding)
				end
			end
			
			if ok and result then
				Core.replace_text(text_selection, result)
				UI.notify("Converted to " .. option.text, vim.log.levels.INFO)
			else
				UI.notify("Conversion failed!", vim.log.levels.ERROR)
			end
		end,
	})
end

-- Close any active preview
function M.close_preview()
	if preview_win and vim.api.nvim_win_is_valid(preview_win.win) then
		vim.api.nvim_win_close(preview_win.win, true)
		preview_win = nil
	end
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