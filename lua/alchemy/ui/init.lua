local Config = require("alchemy.config")

local M = {}

local function create_float(contents, opts)
	opts = opts or {}

	-- Calculate window size
	local width = 0
	for _, line in ipairs(contents) do
		width = math.max(width, #line)
	end
	local height = #contents

	local float_opts = Config.options.ui.float_opts
	float_opts.width = width
	float_opts.height = height

	-- Merge provided opts with defaults
	for k, v in pairs(opts) do
		float_opts[k] = v
	end

	-- Create buffer and window
	local buf = vim.api.nvim_create_buf(false, true)
	-- Escape newlines in contents
	local escaped_contents = {}
	for _, line in ipairs(contents) do
		if type(line) == "string" then
			-- Escape special characters
			line = line:gsub("\n", "\\n")
				:gsub("\r", "\\r")
				:gsub("\t", "\\t")
		end
		table.insert(escaped_contents, tostring(line))
	end
	vim.api.nvim_buf_set_lines(buf, 0, -1, true, escaped_contents)
	local win = vim.api.nvim_open_win(buf, true, float_opts)

	-- Buffer and Window settings
	vim.bo[buf].bufhidden = "delete"
	vim.bo[buf].modifiable = false
	vim.bo[buf].buftype = "nofile"
	vim.wo[win].nu = true
	vim.wo[win].rnu = false
	vim.wo[win].cul = false
    vim.wo[win].signcolumn = "no"

	-- Resize window to fit added line numbers
	vim.api.nvim_win_set_width(win, width + 4)

	-- Add number keymaps for quick line jumps
	for i = 1, #contents do
		vim.api.nvim_buf_set_keymap(buf, "n", tostring(i), "", {
			callback = function()
				vim.api.nvim_win_set_cursor(win, { i, 0 })
			end,
			noremap = true,
			silent = true,
		})
	end

	return buf, win
end

local function format_value(key, value)
	if type(value) == "table" then
		return key .. ">"
	else
		if type(key) == "number" then
			return tostring(value)
		else
			return key .. ": " .. tostring(value)
		end
	end
end

-- Function to handle nested selection

function M.nested_select(data, callback, history, preview_win)
	if next(data) == nil then
		vim.notify("Nothing to select")
		return
	end
	history = history or {}

	-- Get keys from current level
	local keys = {}
	local display_lines = {}
	for k, v in pairs(data) do
		local formated = format_value(k, v)
		table.insert(keys, { key = k, data = v })
		table.insert(display_lines, formated)
	end

	-- First window is relative to cursor
	local is_first = #history == 0

	-- Create float with current options
	local buf, win = create_float(display_lines, {
		relative = is_first and "cursor" or "editor",
		row = is_first and 1 or vim.api.nvim_win_get_position(0)[1],
		col = is_first and 0 or (vim.api.nvim_win_get_position(0)[2] + vim.api.nvim_win_get_width(0) + 2),
	})

	-- Function to show preview
	local function show_preview(next_data)
		-- Close existing preview if any
		if preview_win and vim.api.nvim_win_is_valid(preview_win.win) then
			vim.api.nvim_win_close(preview_win.win, true)
		end

		if type(next_data) == "table" then
			-- Create preview lines
			local preview_lines = {}
			for k, v in pairs(next_data) do
				table.insert(preview_lines, format_value(k, v))
			end

			-- Create preview window
			local preview_buf, preview_window = create_float(preview_lines, {
				relative = "editor",
				row = vim.api.nvim_win_get_position(win)[1],
				col = vim.api.nvim_win_get_position(win)[2] + vim.api.nvim_win_get_width(win) + 2,
			})

			preview_win = { buf = preview_buf, win = preview_window }
			-- Ensure we stay in the original window
			vim.api.nvim_set_current_win(win)
		end
		return preview_win
	end

	-- Set up keymaps
	local opts = { noremap = true, silent = true }

	-- Preview on cursor move
	vim.api.nvim_create_autocmd("CursorMoved", {
		buffer = buf,
		callback = function()
			local line = vim.api.nvim_win_get_cursor(win)[1]
			local selected = keys[line]
			preview_win = show_preview(selected.data)
		end,
	})

	-- Handle selection with <CR>
	vim.api.nvim_buf_set_keymap(buf, "n", "<CR>", "", {
		callback = function()
			local line = vim.api.nvim_win_get_cursor(win)[1]
			local selected = keys[line]
			local next_data = selected.data

			-- Close preview if exists
			if preview_win and vim.api.nvim_win_is_valid(preview_win.win) then
				vim.api.nvim_win_close(preview_win.win, true)
				vim.api.nvim_buf_delete(preview_win.buf, { force = true })
			end

			if type(next_data) == "table" then
				-- Store current selection in history
				table.insert(history, { buf = buf, win = win, data = data })
				-- If next level is a table, continue nesting
				M.nested_select(next_data, callback, history)
			else
				-- If we've reached a leaf, execute callback and clean up
				callback(next_data)
				-- Close current window and buffer
				vim.api.nvim_win_close(win, true)
				vim.api.nvim_buf_delete(buf, { force = true })
				-- Close all history windows and buffers
				for _, prev in ipairs(history) do
					if vim.api.nvim_win_is_valid(prev.win) then
						vim.api.nvim_win_close(prev.win, true)
					end
					if vim.api.nvim_buf_is_valid(prev.buf) then
						vim.api.nvim_buf_delete(prev.buf, { force = true })
					end
				end
			end
		end,
		noremap = true,
		silent = true,
	})

	-- Handle going back with <ESC>
	vim.api.nvim_buf_set_keymap(buf, "n", "<ESC>", "", {
		callback = function()
			-- Close preview if exists
			if preview_win and vim.api.nvim_win_is_valid(preview_win.win) then
				vim.api.nvim_win_close(preview_win.win, true)
				vim.api.nvim_buf_delete(preview_win.buf, { force = true })
			end

			-- Close current window and buffer
			vim.api.nvim_win_close(win, true)
			vim.api.nvim_buf_delete(buf, { force = true })

			-- Go back to previous window if exists
			if #history > 0 then
				local prev = table.remove(history)
				if vim.api.nvim_win_is_valid(prev.win) then
					vim.api.nvim_set_current_win(prev.win)
				end
			end
		end,
		noremap = true,
		silent = true,
	})
end

return M
