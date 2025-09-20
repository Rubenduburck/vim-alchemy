-- Beautiful floating window UI components for alchemy
-- Provides rich visual feedback with syntax highlighting and animations

local M = {}

local selection_ns = vim.api.nvim_create_namespace("alchemy_selection_highlight")
local selection_state = { bufnr = nil }

local function ensure_highlight_group()
	if pcall(vim.api.nvim_get_hl, 0, { name = "AlchemySelectionHighlight" }) then
		return
	end
	vim.api.nvim_set_hl(0, "AlchemySelectionHighlight", { link = "Visual" })
end

function M.clear_selection_highlight()
	if selection_state.bufnr and vim.api.nvim_buf_is_valid(selection_state.bufnr) then
		vim.api.nvim_buf_clear_namespace(selection_state.bufnr, selection_ns, 0, -1)
	end
	selection_state.bufnr = nil
end

function M.highlight_selection(selection)
	M.clear_selection_highlight()
	if
		not selection
		or selection.start_line == nil
		or selection.start_col == nil
		or selection.end_line == nil
		or selection.end_col == nil
	then
		return
	end

	ensure_highlight_group()
	local bufnr = vim.api.nvim_get_current_buf()
	selection_state.bufnr = bufnr

	local start_line = selection.start_line
	local end_line = selection.end_line
	local start_col = selection.start_col
	local end_col = selection.end_col

	for line = start_line, end_line do
		local line_start = (line == start_line) and start_col or 0
		local line_end
		if line == end_line then
			line_end = math.max(line_start + 1, end_col)
		else
			line_end = -1
		end
		vim.api.nvim_buf_add_highlight(bufnr, selection_ns, "AlchemySelectionHighlight", line, line_start, line_end)
	end
end

function M.compute_window_geometry(selection, width, height)
	local total_lines = vim.o.lines
	local total_cols = vim.o.columns

	width = math.max(12, math.min(width or 40, total_cols - 4))
	height = math.max(3, math.min(height or 10, total_lines - 4))

	local win = vim.api.nvim_get_current_win()

	local function screen_position(line, col)
		local ok, pos = pcall(vim.fn.screenpos, win, line, math.max(1, col))
		if ok and pos.row and pos.row > 0 then
			return pos.row - 1, pos.col - 1
		end
		return nil, nil
	end

	local anchor_row
	local anchor_col

	if selection and selection.end_line then
		local line = selection.end_line + 1
		anchor_row = select(1, screen_position(line, 1))
		local start_line = (selection.start_line or selection.end_line) + 1
		anchor_col = select(2, screen_position(start_line, (selection.start_col or 0) + 1))
	end

	if not anchor_row then
		local cursor = vim.api.nvim_win_get_cursor(win)
		anchor_row, anchor_col = screen_position(cursor[1], cursor[2] + 1)
		if not anchor_col then
			anchor_col = cursor[2]
		end
	end

	if not anchor_row then
		local win_pos = vim.fn.win_screenpos(win)
		local view = vim.fn.winsaveview()
		local cursor = vim.api.nvim_win_get_cursor(win)
		local line = selection and selection.end_line and (selection.end_line + 1) or cursor[1]
		local cursor_col = selection and selection.start_col or cursor[2]
		local topline = view.topline or 1
		local relative_row = line - topline
		anchor_row = win_pos[1] - 1 + math.max(0, relative_row)
		anchor_col = win_pos[2] - 1 + (cursor_col or 0)
	end

	anchor_col = math.max(0, (anchor_col or 0) - 2)

	local desired_row = (anchor_row or 0) + 1
	local row = desired_row

	if row + height > total_lines - 2 then
		local shrink = (total_lines - 2) - row
		if shrink >= 3 then
			height = shrink
		else
			row = math.max(0, total_lines - height - 2)
		end
	end

	row = math.max(0, row)
	if row + height > total_lines - 2 then
		row = math.max(0, total_lines - height - 2)
	end

	local col = anchor_col or 0
	if col + width > total_cols - 2 then
		col = math.max(0, total_cols - width - 2)
	end

	return row, col, width, height
end

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
		row = opts.row
		col = opts.col
		if row == nil or col == nil then
			local position = opts.position or "center"
			if position == "bottom" then
				row = math.max(0, vim.o.lines - height - 4)
				col = math.floor((vim.o.columns - width) / 2)
			elseif position == "bottom_right" then
				row = math.max(0, vim.o.lines - height - 4)
				col = math.max(0, vim.o.columns - width - 4)
			elseif position == "bottom_left" then
				row = math.max(0, vim.o.lines - height - 4)
				col = 2
			else
				row = math.floor((vim.o.lines - height) / 2)
				col = math.floor((vim.o.columns - width) / 2)
			end
		end
		row = row or math.floor((vim.o.lines - height) / 2)
		col = col or math.floor((vim.o.columns - width) / 2)
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
	vim.api.nvim_buf_set_option(buf, "filetype", opts.filetype or "alchemy_float")

	local function close_window()
		if vim.api.nvim_win_is_valid(win) then
			vim.api.nvim_win_close(win, true)
		end
		if opts.on_close then
			opts.on_close()
		end
	end

	local close_keys = opts.close_keys or { "q", "<Esc>", "<C-c>" }
	for _, key in ipairs(close_keys) do
		vim.keymap.set("n", key, close_window, {
			buffer = buf,
			nowait = true,
			silent = true,
			desc = "Close alchemy window",
		})
	end

	return {
		buf = buf,
		win = win,
		width = width,
		height = height,
		close = close_window,
	}
end

-- Create a preview window showing before/after
function M.create_preview(original_text, converted_text, opts)
	opts = opts or {}

	local function safe_call(cb, ...)
		if not cb then
			return
		end
		local ok, err = pcall(cb, ...)
		if not ok then
			M.notify(tostring(err), vim.log.levels.ERROR)
		end
	end

	local previous_win = vim.api.nvim_get_current_win()

	if opts.selection then
		M.highlight_selection(opts.selection)
	end

	local float = M.create_float({
		title = opts.title or " 🔄 Conversion Preview ",
		width = opts.width or math.min(80, vim.o.columns - 10),
		height = opts.height or math.min(20, vim.o.lines - 5),
		border = "double",
		position = opts.position,
		row = opts.row,
		col = opts.col,
		cursor_relative = opts.cursor_relative,
		on_close = function()
			safe_call(opts.on_close)
			M.clear_selection_highlight()
			if vim.api.nvim_win_is_valid(previous_win) then
				vim.api.nvim_set_current_win(previous_win)
			end
		end,
	})

	local lines = {
		"╭─ Original ─╮",
		"",
	}

	for _, line in ipairs(M.wrap_text(original_text, float.width - 4)) do
		table.insert(lines, "  " .. line)
	end

	table.insert(lines, "")
	table.insert(lines, "╰────────────╯")
	table.insert(lines, "")
	table.insert(lines, "╭─ Converted ─╮")
	table.insert(lines, "")

	for _, line in ipairs(M.wrap_text(converted_text, float.width - 4)) do
		table.insert(lines, "  " .. line)
	end

	table.insert(lines, "")
	table.insert(lines, "╰─────────────╯")
	table.insert(lines, "")
	table.insert(lines, "Press Enter to apply, q/Esc to cancel")

	vim.api.nvim_buf_set_lines(float.buf, 0, -1, false, lines)
	vim.api.nvim_buf_set_option(float.buf, "modifiable", false)

	vim.api.nvim_buf_call(float.buf, function()
		vim.cmd("syntax match AlchemyHeader /╭─.*─╮/")
		vim.cmd("syntax match AlchemyHeader /╰─.*─╯/")
		vim.cmd("highlight AlchemyHeader guifg=#7aa2f7 gui=bold")
		vim.cmd("syntax match AlchemyInstruction /Press Enter.*/")
		vim.cmd("highlight AlchemyInstruction guifg=#9ece6a gui=italic")
	end)

	local function apply_result()
		safe_call(opts.on_pre_confirm)
		float.close()
		safe_call(opts.on_confirm)
	end

	local function cancel_preview()
		float.close()
	end

	local map_opts = { buffer = float.buf, silent = true, nowait = true }
	vim.keymap.set("n", "<CR>", apply_result, map_opts)
	vim.keymap.set("n", "<Space>", apply_result, map_opts)
	vim.keymap.set("n", "q", cancel_preview, map_opts)
	vim.keymap.set("n", "<Esc>", cancel_preview, map_opts)
	vim.keymap.set("n", "<C-c>", cancel_preview, map_opts)

	vim.api.nvim_set_current_win(float.win)

	return float
end

-- Create a compact floating menu with hjkl navigation
function M.create_selector(items, opts)
	opts = opts or {}

	if not items or vim.tbl_isempty(items) then
		M.notify(opts.empty_message or "No options available", vim.log.levels.WARN)
		return nil
	end

	local origin_win = vim.api.nvim_get_current_win()
	local formatted, max_width = {}, 0
	local max_width_limit = math.max(opts.max_width or math.floor(vim.o.columns * 0.6), 20)

	for idx, item in ipairs(items) do
		local raw_text = item.text or item.name or tostring(item)
		local icon = item.icon or M.get_icon_for_type(item.type)
		local available = math.max(max_width_limit - 6, 10)
		local display_text = raw_text
		if vim.fn.strdisplaywidth(raw_text) > available then
			display_text = vim.fn.strcharpart(raw_text, 0, available - 1) .. "…"
		end
		local line = string.format(" %s %s", icon, display_text)
		formatted[idx] = { item = item, line = line }
		max_width = math.max(max_width, vim.fn.strdisplaywidth(line))
	end

	local width = math.min(max_width + 2, max_width_limit)
	local height = math.min(#formatted + 2, opts.max_height or 15)

	if not opts.row or not opts.col then
		local geom_row, geom_col, geom_width, geom_height = M.compute_window_geometry(opts.selection, width, height)
		width = math.min(width, geom_width)
		height = math.min(height, geom_height)
		opts.row = opts.row or geom_row
		opts.col = opts.col or geom_col
	end

	if opts.selection then
		M.highlight_selection(opts.selection)
	end

	local preview_float = nil
	local function cleanup_preview()
		if not preview_float then
			return
		end
		if preview_float.close then
			preview_float.close()
		elseif preview_float.win and vim.api.nvim_win_is_valid(preview_float.win) then
			vim.api.nvim_win_close(preview_float.win, true)
		end
		preview_float = nil
	end

	local highlight_ns = vim.api.nvim_create_namespace("alchemy_selector")
	local closed = false

	local function safe_call(cb, ...)
		if not cb then
			return true
		end
		local ok, err = pcall(cb, ...)
		if not ok then
			M.notify(tostring(err), vim.log.levels.ERROR)
		end
		return ok
	end

	local float = M.create_float({
		title = opts.title,
		width = width,
		height = height,
		cursor_relative = opts.cursor_relative,
		border = opts.border or "single",
		position = opts.position,
		row = opts.row,
		col = opts.col,
		close_keys = {},
		on_close = function()
			if closed then
				return
			end
			closed = true
			cleanup_preview()
			safe_call(opts.on_close)
			M.clear_selection_highlight()
			if vim.api.nvim_win_is_valid(origin_win) then
				vim.api.nvim_set_current_win(origin_win)
			end
		end,
	})

	local buf, win = float.buf, float.win
	vim.api.nvim_buf_set_lines(
		buf,
		0,
		-1,
		false,
		vim.tbl_map(function(entry)
			return entry.line
		end, formatted)
	)
	vim.api.nvim_buf_set_option(buf, "modifiable", false)

	local current = opts.initial_index or 1
	if current < 1 or current > #formatted then
		current = 1
	end

	local function highlight_current()
		vim.api.nvim_buf_clear_namespace(buf, highlight_ns, 0, -1)
		vim.api.nvim_buf_add_highlight(buf, highlight_ns, "Visual", current - 1, 0, -1)
	end

	local function focus_current()
		if vim.api.nvim_win_is_valid(win) then
			vim.api.nvim_win_set_cursor(win, { current, 0 })
		end
		highlight_current()
	end

	local function open_preview()
		cleanup_preview()
		if not opts.on_preview then
			return
		end
		local ok, preview_content = pcall(opts.on_preview, formatted[current].item, current)
		if not ok then
			M.notify("Preview failed: " .. tostring(preview_content), vim.log.levels.ERROR)
			return
		end
		if not preview_content or preview_content == "" then
			return
		end
		if
			type(preview_content) == "table"
			and preview_content.win
			and vim.api.nvim_win_is_valid(preview_content.win)
		then
			preview_float = preview_content
			return
		end
		local preview_text = type(preview_content) == "table" and vim.inspect(preview_content)
			or tostring(preview_content)
		preview_float = M.create_preview_popup(preview_text, {
			anchor_win = win,
			anchor_row = current - 1,
		})
	end

	local function move_cursor(delta)
		current = current + delta
		if current < 1 then
			current = #formatted
		elseif current > #formatted then
			current = 1
		end
		focus_current()
		open_preview()
	end

	local function close_menu()
		if closed then
			return
		end
		float.close()
	end

	local function select_current()
		local entry = formatted[current].item

		if opts.on_pre_select then
			local ok, proceed = pcall(opts.on_pre_select, entry, current)
			if not ok then
				M.notify(tostring(proceed), vim.log.levels.ERROR)
				return
			end
			if proceed == false then
				return
			end
		end

		close_menu()
		if opts.on_select then
			vim.schedule(function()
				safe_call(opts.on_select, entry, current)
			end)
		end
	end

	local key_opts = { buffer = buf, silent = true, nowait = true }
	for _, key in ipairs({ "j", "<Down>" }) do
		vim.keymap.set("n", key, function()
			move_cursor(1)
		end, key_opts)
	end
	for _, key in ipairs({ "k", "<Up>" }) do
		vim.keymap.set("n", key, function()
			move_cursor(-1)
		end, key_opts)
	end
	vim.keymap.set("n", "<C-n>", function()
		move_cursor(1)
	end, key_opts)
	vim.keymap.set("n", "<C-p>", function()
		move_cursor(-1)
	end, key_opts)

	for _, key in ipairs({ "<CR>", "<Space>", "l" }) do
		vim.keymap.set("n", key, select_current, key_opts)
	end

	for _, key in ipairs({ "q", "<Esc>", "<C-c>" }) do
		vim.keymap.set("n", key, close_menu, key_opts)
	end

	if opts.on_back then
		vim.keymap.set("n", "h", function()
			close_menu()
			vim.schedule(function()
				safe_call(opts.on_back)
			end)
		end, key_opts)
	end

	for i = 1, math.min(9, #formatted) do
		vim.keymap.set("n", tostring(i), function()
			current = i
			focus_current()
			open_preview()
			select_current()
		end, key_opts)
	end

	focus_current()
	open_preview()
	vim.api.nvim_set_current_win(win)

	return float
end

-- Create a small preview popup next to the main menu
function M.create_preview_popup(content, opts)
	opts = opts or {}

	local lines
	if type(content) == "table" then
		lines = {}
		for i, line in ipairs(content) do
			lines[i] = tostring(line)
		end
	else
		lines = vim.split(tostring(content), "\n")
	end

	local width = 0
	for _, line in ipairs(lines) do
		width = math.max(width, vim.fn.strdisplaywidth(line))
	end
	width = math.min(width + 2, opts.max_width or 60)
	local height = math.min(#lines + 2, opts.max_height or 15)

	local anchor_win = opts.anchor_win or vim.api.nvim_get_current_win()
	local anchor = vim.api.nvim_win_get_config(anchor_win)
	local function resolve(value)
		if type(value) == "table" then
			return value[1] or 0
		end
		return value or 0
	end

	local anchor_width = anchor.width or vim.api.nvim_win_get_width(anchor_win)
	local row = resolve(anchor.row) + (opts.anchor_row or 0)
	local col = resolve(anchor.col) + anchor_width + (opts.col_offset or 2)

	local float = M.create_float({
		relative = anchor.relative or "editor",
		row = row,
		col = col,
		width = width,
		height = height,
		border = opts.border or "single",
		title = opts.title or " Preview ",
		close_keys = {},
	})

	local padded = {}
	for _, line in ipairs(lines) do
		table.insert(padded, " " .. line)
	end
	vim.api.nvim_buf_set_lines(float.buf, 0, -1, false, padded)
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
