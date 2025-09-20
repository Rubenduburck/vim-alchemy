-- Core module for the new alchemy plugin
-- Handles text selection, CLI communication, and result processing

local M = {}

local Config = require("alchemy.config")

local VISUAL_MODES = {
	["v"] = true,
	["V"] = true,
	["\22"] = true,
}

local function is_visual_mode(mode)
	return VISUAL_MODES[mode] or false
end

local function get_visual_marks()
	local start_pos = vim.fn.getpos("'<")
	local end_pos = vim.fn.getpos("'>")

	local start_line, start_col = start_pos[2], start_pos[3]
	local end_line, end_col = end_pos[2], end_pos[3]

	if start_line == 0 and end_line == 0 then
		return nil
	end

	if start_line == end_line and start_col == end_col then
		return nil
	end

	if start_line > end_line or (start_line == end_line and start_col > end_col) then
		start_line, end_line = end_line, start_line
		start_col, end_col = end_col, start_col
	end

	return {
		start_line = start_line,
		start_col = start_col,
		end_line = end_line,
		end_col = end_col,
	}
end

local function cursor_within_marks(range, mode)
	if not range then
		return false
	end

	local cursor = vim.api.nvim_win_get_cursor(0)
	local cursor_line = cursor[1]
	local cursor_col = cursor[2] + 1 -- convert to 1-based

	if cursor_line < range.start_line or cursor_line > range.end_line then
		return false
	end

	if mode == "V" then
		return true
	end

	if cursor_line == range.start_line and cursor_col < range.start_col then
		return false
	end

	local effective_end_col = range.end_col
	if vim.o.selection ~= "inclusive" and effective_end_col > range.start_col then
		effective_end_col = effective_end_col - 1
	end

	if cursor_line == range.end_line and cursor_col > effective_end_col then
		return false
	end

	return true
end

local function exit_visual_mode()
	local esc = vim.api.nvim_replace_termcodes("<Esc>", true, false, true)
	vim.api.nvim_feedkeys(esc, "nx", false)
end

-- Get text under cursor or visual selection
function M.get_text_selection()
	local mode = vim.fn.mode()

	if is_visual_mode(mode) then
		local selection = M.get_visual_selection({ mode = mode })
		exit_visual_mode()
		return selection
	end

	local last_visual_mode = vim.fn.visualmode()
	if is_visual_mode(last_visual_mode) then
		local marks = get_visual_marks()
		if cursor_within_marks(marks, last_visual_mode) then
			exit_visual_mode()
			return M.get_visual_selection({ mode = last_visual_mode, marks = marks })
		end
	end

	return M.get_word_under_cursor()
end

-- Get visual selection (existing implementation)

local function sanitize_marks(marks)
	if marks then
		return marks.start_line, marks.start_col, marks.end_line, marks.end_col
	end
	local start_pos = vim.fn.getpos("'<")
	local end_pos = vim.fn.getpos("'>")
	return start_pos[2], start_pos[3], end_pos[2], end_pos[3]
end

function M.get_visual_selection(opts)
	ops = opts or {}
	local line_start, column_start, line_end, column_end = sanitize_marks(opts.marks)

	if not line_start or line_start == 0 or not line_end or line_end == 0 then
		return {
			text = "",
			start_line = nil,
			start_col = nil,
			end_line = nil,
			end_col = nil,
		}
	end

	local lines = vim.fn.getline(line_start, line_end)

	if type(lines) == "string" then
		lines = { lines }
	end

	if #lines == 0 then
		return {
			text = "",
			start_line = nil,
			start_col = nil,
			end_line = nil,
			end_col = nil,
		}
	end

	local last_line_length = #lines[#lines]
	column_end = math.min(column_end, last_line_length)

	if #lines > 0 then
		lines[#lines] = lines[#lines]:sub(1, column_end - (vim.o.selection == "inclusive" and 0 or 1))
		lines[1] = lines[1]:sub(column_start)
	end

	return {
		text = table.concat(lines, "\n"),
		start_line = line_start - 1,
		start_col = column_start - 1,
		end_line = line_end - 1,
		end_col = column_end,
	}
end

-- Get WORD under cursor
function M.get_word_under_cursor()
	local cursor_pos = vim.api.nvim_win_get_cursor(0)
	local line = vim.api.nvim_get_current_line()
	local col = cursor_pos[2]

	-- Find start and end of WORD (non-whitespace sequence)
	local start_col = col
	local end_col = col

	-- Find start of word (move backwards)
	while start_col > 0 and line:sub(start_col, start_col):match("%S") do
		start_col = start_col - 1
	end
	start_col = start_col + 1 -- Move to first non-whitespace char

	-- Find end of word (move forwards)
	while end_col <= #line and line:sub(end_col + 1, end_col + 1):match("%S") do
		end_col = end_col + 1
	end

	local word = line:sub(start_col, end_col)

	return {
		text = word,
		start_line = cursor_pos[1] - 1,
		start_col = start_col - 1,
		end_line = cursor_pos[1] - 1,
		end_col = end_col,
	}
end

-- Replace selection or word with new text
function M.replace_text(selection, new_text)
	local bufnr = vim.api.nvim_get_current_buf()
	local lines = vim.split(new_text, "\n", { plain = true })

	if
		selection.start_line == nil
		or selection.start_col == nil
		or selection.end_line == nil
		or selection.end_col == nil
	then
		vim.api.nvim_put(lines, "c", true, true)
		return
	end

	vim.api.nvim_buf_set_text(
		bufnr,
		selection.start_line,
		selection.start_col,
		selection.end_line,
		selection.end_col,
		lines
	)
end

-- Execute CLI command and parse response (JSON for classify, plain text for convert)
function M.execute_cli(cmd_args, expect_json)
	expect_json = expect_json == nil and true or expect_json -- Default to true

	-- Build the command - alchemy binary + all arguments
	local cmd = { Config.options.cli.bin or "alchemy" }
	vim.list_extend(cmd, cmd_args)
	
	local result = vim.fn.systemlist(cmd)
	local output = table.concat(result, "\n")

	if vim.v.shell_error ~= 0 then
		if output:match("command not found") or output:match("No such file") then
			error("alchemy binary not found. Please run 'make install' in the plugin directory.")
		else
			error("CLI command failed: " .. output)
		end
	end

	if expect_json then
		local ok, parsed = pcall(vim.json.decode, output)
		if not ok then
			error("Failed to parse CLI response: " .. output)
		end
		return parsed
	else
		return output
	end
end

-- Classify text and return all classifications
function M.classify(text)
	local args = { "-l", "classify", text }
	return M.execute_cli(args)
end

-- Convert with explicit input/output encodings
function M.convert(text, input_encoding, output_encoding)
	local args = { "convert", "-o", output_encoding }

	if input_encoding then
		table.insert(args, "-i")
		table.insert(args, input_encoding)
	end

	table.insert(args, text)

	-- When both input and output encodings are specified, the CLI returns plain text
	-- Otherwise it might return JSON, so we try plain text first
	local ok, result = pcall(M.execute_cli, args, false)
	if ok and result and result ~= "" then
		return result
	end
	
	-- If plain text failed, try JSON format
	ok, result = pcall(M.execute_cli, args, true)
	if ok and type(result) == "table" then
		-- Extract the converted value from JSON
		for _, conversions in pairs(result) do
			if conversions[output_encoding] then
				return conversions[output_encoding].output
			end
		end
	end
	
	error("No conversion result found")
end

-- Auto-classify and convert (no input encoding specified)
function M.classify_and_convert(text, output_encoding)
	-- Try simple conversion first (should auto-classify)
	local simple_args = { "convert", "-o", output_encoding, text }
	local ok, result = pcall(M.execute_cli, simple_args, false)
	
	if ok and result and result ~= "" then
		return result
	end
	
	-- If simple conversion failed, try with JSON format to get all classifications
	local json_args = { "-l", "convert", "-o", output_encoding, text }
	ok, result = pcall(M.execute_cli, json_args, true)
	
	if ok and type(result) == "table" then
		-- Find the best conversion result from the JSON response
		for _, conversions in pairs(result) do
			if conversions[output_encoding] and conversions[output_encoding].output then
				local output = conversions[output_encoding].output
				if output ~= "" then
					return output
				end
			end
		end
	end
	
	error("No valid conversion result found")
end

return M
