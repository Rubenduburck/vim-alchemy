-- Beautiful floating window UI components for alchemy
-- Provides rich visual feedback with syntax highlighting and animations

local M = {}

-- Create a floating window with proper error handling
function M.create_float(opts)
	opts = opts or {}
	
	-- Calculate dimensions with bounds checking
	local width = opts.width or math.min(80, vim.o.columns - 4)
	local height = opts.height or math.min(20, vim.o.lines - 4)
	
	-- Ensure minimum size
	width = math.max(width, 10)
	height = math.max(height, 3)
	
	-- Calculate position
	local row, col
	if opts.cursor_relative then
		-- Position relative to cursor
		local cursor = vim.api.nvim_win_get_cursor(0)
		row = cursor[1] - 1 -- Convert to 0-based
		col = cursor[2]
		
		-- Adjust if window would go off-screen
		if row + height > vim.o.lines - 2 then
			row = vim.o.lines - height - 2
		end
		if col + width > vim.o.columns - 2 then
			col = vim.o.columns - width - 2
		end
	else
		-- Centered
		row = opts.row or math.floor((vim.o.lines - height) / 2)
		col = opts.col or math.floor((vim.o.columns - width) / 2)
	end
	
	-- Create buffer
	local buf = vim.api.nvim_create_buf(false, true)
	
	-- Window options with safe defaults
	local win_opts = {
		relative = opts.relative or "editor",
		width = width,
		height = height,
		row = row,
		col = col,
		style = "minimal",
		border = opts.border or "single",
	}
	
	-- Only add title if supported
	if opts.title and vim.fn.has("nvim-0.9") == 1 then
		win_opts.title = opts.title
		win_opts.title_pos = opts.title_pos or "center"
	end
	
	-- Create window with error handling
	local ok, win = pcall(vim.api.nvim_open_win, buf, false, win_opts)
	if not ok then
		vim.api.nvim_buf_delete(buf, { force = true })
		error("Failed to create window: " .. tostring(win))
	end
	
	-- Set window options safely
	pcall(vim.api.nvim_win_set_option, win, "winblend", opts.winblend or 0)
	pcall(vim.api.nvim_win_set_option, win, "cursorline", true)
	pcall(vim.api.nvim_win_set_option, win, "number", false)
	pcall(vim.api.nvim_win_set_option, win, "relativenumber", false)
	
	-- Set buffer options
	vim.api.nvim_buf_set_option(buf, "bufhidden", "wipe")
	vim.api.nvim_buf_set_option(buf, "modifiable", opts.modifiable ~= false)
	vim.api.nvim_buf_set_option(buf, "buftype", "nofile")
	
	-- Add close keymaps
	local close_keys = opts.close_keys or { "q", "<Esc>", "<C-c>" }
	for _, key in ipairs(close_keys) do
		vim.api.nvim_buf_set_keymap(buf, "n", key, "", {
			noremap = true,
			silent = true,
			callback = function()
				if vim.api.nvim_win_is_valid(win) then
					vim.api.nvim_win_close(win, true)
				end
				-- Call cleanup callback if provided
				if opts.on_close then
					opts.on_close()
				end
			end,
		})
	end
	
	return {
		buf = buf,
		win = win,
		width = width,
		height = height,
	}
end

-- Create a preview window showing before/after
function M.create_preview(original_text, converted_text, opts)
	opts = opts or {}
	
	local float = M.create_float({
		title = opts.title or " 🔄 Conversion Preview ",
		width = opts.width or math.min(80, vim.o.columns - 10),
		height = opts.height or math.min(20, vim.o.lines - 5),
		border = "double",
	})
	
	-- Create content with nice formatting
	local lines = {
		"╭─ Original ─╮",
		"",
	}
	
	-- Add original text (wrapped if needed)
	local original_lines = M.wrap_text(original_text, float.width - 4)
	for _, line in ipairs(original_lines) do
		table.insert(lines, "  " .. line)
	end
	
	table.insert(lines, "")
	table.insert(lines, "╰────────────╯")
	table.insert(lines, "")
	table.insert(lines, "╭─ Converted ─╮")
	table.insert(lines, "")
	
	-- Add converted text (wrapped if needed)
	local converted_lines = M.wrap_text(converted_text, float.width - 4)
	for _, line in ipairs(converted_lines) do
		table.insert(lines, "  " .. line)
	end
	
	table.insert(lines, "")
	table.insert(lines, "╰─────────────╯")
	table.insert(lines, "")
	table.insert(lines, "Press Enter to apply, q/Esc to cancel")
	
	-- Set content
	vim.api.nvim_buf_set_lines(float.buf, 0, -1, false, lines)
	
	-- Syntax highlighting
	vim.api.nvim_buf_call(float.buf, function()
		-- Highlight headers
		vim.cmd("syntax match AlchemyHeader /╭─.*─╮/")
		vim.cmd("syntax match AlchemyHeader /╰─.*─╯/")
		vim.cmd("highlight AlchemyHeader guifg=#7aa2f7 gui=bold")
		
		-- Highlight the instruction line
		vim.cmd("syntax match AlchemyInstruction /Press Enter.*/")
		vim.cmd("highlight AlchemyInstruction guifg=#9ece6a gui=italic")
		
		-- Highlight the content based on type
		if opts.original_syntax then
			-- Apply syntax highlighting for original content
		end
		if opts.converted_syntax then
			-- Apply syntax highlighting for converted content
		end
	end)
	
	-- Add Enter key to apply
	vim.api.nvim_buf_set_keymap(float.buf, "n", "<CR>", "", {
		noremap = true,
		silent = true,
		callback = function()
			vim.api.nvim_win_close(float.win, true)
			if opts.on_confirm then
				opts.on_confirm()
			end
		end,
	})
	
	-- Focus the preview window
	vim.api.nvim_set_current_win(float.win)
	
	return float
end

-- Create a compact floating menu with hjkl navigation
function M.create_selector(items, opts)
	opts = opts or {}
	
	-- Calculate compact dimensions
	local max_width = 0
	for _, item in ipairs(items) do
		local text = item.text or item.name or tostring(item)
		max_width = math.max(max_width, #text + 10) -- account for icon and padding
	end
	
	local width = math.min(max_width, 50)
	local height = math.min(#items + 2, 15) -- compact height
	
	-- Preview window reference
	local preview_win = nil
	
	-- Cleanup function for preview windows
	local function cleanup_preview()
		if preview_win and vim.api.nvim_win_is_valid(preview_win.win) then
			vim.api.nvim_win_close(preview_win.win, true)
			preview_win = nil
		end
	end
	
	local float = M.create_float({
		title = opts.title,
		width = width,
		height = height,
		cursor_relative = opts.cursor_relative ~= false, -- default to cursor relative
		border = "single",
		on_close = cleanup_preview,
	})
	
	-- Format items compactly
	local lines = {}
	for i, item in ipairs(items) do
		local icon = item.icon or M.get_icon_for_type(item.type)
		local text = item.text or item.name or tostring(item)
		
		-- Truncate long text
		if #text > width - 6 then
			text = text:sub(1, width - 9) .. "..."
		end
		
		table.insert(lines, string.format(" %s %s", icon, text))
	end
	
	-- Set content
	vim.api.nvim_buf_set_lines(float.buf, 0, -1, false, lines)
	
	-- Make read-only
	vim.api.nvim_buf_set_option(float.buf, "modifiable", false)
	
	-- Track current selection
	local current_line = 1
	vim.api.nvim_win_set_cursor(float.win, { current_line, 0 })
	
	-- Update selection and show preview
	local function update_selection(new_line)
		current_line = new_line
		if current_line < 1 then
			current_line = #items
		elseif current_line > #items then
			current_line = 1
		end
		
		-- Move cursor
		vim.api.nvim_win_set_cursor(float.win, { current_line, 0 })
		
		-- Close previous preview
		cleanup_preview()
		
		-- Show new preview if enabled
		if opts.on_preview then
			local preview_content = opts.on_preview(items[current_line], current_line)
			if preview_content then
				preview_win = M.create_preview_popup(preview_content, {
					anchor_win = float.win,
					anchor_row = current_line - 1,
				})
			end
		end
	end
	
	-- Navigation with hjkl
	vim.api.nvim_buf_set_keymap(float.buf, "n", "j", "", {
		noremap = true,
		silent = true,
		callback = function() update_selection(current_line + 1) end,
	})
	
	vim.api.nvim_buf_set_keymap(float.buf, "n", "k", "", {
		noremap = true,
		silent = true,
		callback = function() update_selection(current_line - 1) end,
	})
	
	-- Enter/l to select
	local function select_current()
		-- Close preview
		cleanup_preview()
		
		vim.api.nvim_win_close(float.win, true)
		if opts.on_select then
			opts.on_select(items[current_line], current_line)
		end
	end
	
	vim.api.nvim_buf_set_keymap(float.buf, "n", "<CR>", "", {
		noremap = true,
		silent = true,
		callback = select_current,
	})
	
	vim.api.nvim_buf_set_keymap(float.buf, "n", "l", "", {
		noremap = true,
		silent = true,
		callback = select_current,
	})
	
	-- h to go back (if parent callback provided)
	if opts.on_back then
		vim.api.nvim_buf_set_keymap(float.buf, "n", "h", "", {
			noremap = true,
			silent = true,
			callback = function()
				-- Close preview
				cleanup_preview()
				
				vim.api.nvim_win_close(float.win, true)
				opts.on_back()
			end,
		})
	end
	
	-- Number key shortcuts
	for i = 1, math.min(9, #items) do
		vim.api.nvim_buf_set_keymap(float.buf, "n", tostring(i), "", {
			noremap = true,
			silent = true,
			callback = function()
				update_selection(i)
				select_current()
			end,
		})
	end
	
	-- Focus the window
	vim.api.nvim_set_current_win(float.win)
	
	-- Initial preview
	if opts.on_preview then
		update_selection(1)
	end
	
	return float
end

-- Create a small preview popup next to the main menu
function M.create_preview_popup(content, opts)
	opts = opts or {}
	
	-- Small preview window
	local lines = vim.split(content, "\n")
	local width = 0
	for _, line in ipairs(lines) do
		width = math.max(width, #line)
	end
	width = math.min(width + 2, 40)
	
	local height = math.min(#lines + 2, 10)
	
	-- Position to the right of the anchor
	local anchor_info = vim.api.nvim_win_get_config(opts.anchor_win)
	local row = anchor_info.row + (opts.anchor_row or 0)
	local col = anchor_info.col + anchor_info.width + 2
	
	local float = M.create_float({
		width = width,
		height = height,
		row = row,
		col = col,
		border = "single",
		title = " Preview ",
	})
	
	-- Add content with padding
	local padded_lines = {}
	for _, line in ipairs(lines) do
		table.insert(padded_lines, " " .. line)
	end
	
	vim.api.nvim_buf_set_lines(float.buf, 0, -1, false, padded_lines)
	vim.api.nvim_buf_set_option(float.buf, "modifiable", false)
	
	return float
end

-- Helper function to wrap text
function M.wrap_text(text, width)
	if #text <= width then
		return { text }
	end
	
	local lines = {}
	local current_line = ""
	
	for word in text:gmatch("%S+") do
		if #current_line + #word + 1 <= width then
			if #current_line > 0 then
				current_line = current_line .. " " .. word
			else
				current_line = word
			end
		else
			if #current_line > 0 then
				table.insert(lines, current_line)
			end
			current_line = word
		end
	end
	
	if #current_line > 0 then
		table.insert(lines, current_line)
	end
	
	return lines
end

-- Get icon for encoding type
function M.get_icon_for_type(type_name)
	local icons = {
		hex = "🔤",
		int = "🔢",
		base64 = "📝",
		base58 = "🔠",
		bin = "💾",
		binary = "💾",
		utf8 = "📄",
		ascii = "🔤",
		bytes = "📊",
		hash = "🔐",
		sha256 = "🔐",
		sha512 = "🔐",
		md5 = "🔐",
		array = "📚",
		convert = "🔄",
		operation = "⚡",
	}
	
	if type_name then
		return icons[type_name:lower()] or "🎯"
	end
	return "🎯"
end

-- Simple notification using vim.notify
function M.notify(message, level, opts)
	opts = opts or {}
	
	-- Just use vim.notify which is more reliable
	vim.notify(message, level or vim.log.levels.INFO, {
		title = opts.title or "Alchemy",
	})
end

return M