local M = {}

function M.merge_opts(defaults, opts)
	if not defaults then
		return opts or {}
	end
	if not opts then
		return vim.deepcopy(defaults)
	end
	return vim.tbl_deep_extend("force", defaults, opts)
end

function M.get_visual_selection()
	local line_start, column_start = vim.fn.getpos("'<")[2], vim.fn.getpos("'<")[3]
	local line_end, column_end = vim.fn.getpos("'>")[2], vim.fn.getpos("'>")[3]
	local lines = vim.fn.getline(line_start, line_end)
	if #lines == 0 then
		return "", nil, nil, nil, nil
	end

	-- Get the actual length of the last line
	local last_line_length = #lines[#lines]
	-- Adjust column_end to not exceed the actual line length
	column_end = math.min(column_end, last_line_length)

	lines[#lines] = lines[#lines]:sub(1, column_end - (vim.o.selection == "inclusive" and 0 or 1))
	lines[1] = lines[1]:sub(column_start)
	return {
		text = table.concat(lines, "\n"),
		start_line = line_start - 1,
		start_col = column_start - 1,
		end_line = line_end - 1,
		end_col = column_end,
	}
end

-- If valid selection, replace it with text
-- Otherwise, insert text at the cursor position
function M.replace_selection(bufnr, selection, text)
	if
		selection.start_line == nil
		or selection.start_col == nil
		or selection.end_line == nil
		or selection.end_col == nil
	then
		vim.api.nvim_put({ text }, "c", true, true)
		return
	else
		vim.api.nvim_buf_set_text(
			bufnr,
			selection.start_line,
			selection.start_col,
			selection.end_line,
			selection.end_col,
			{ text }
		)
	end
end

function M.collapse_on_key(data, key)
	-- If not a table, return as is
	if type(data) ~= "table" then
		return data
	end

	if data[key] then
		return M.collapse_on_key(data[key], key)
	end

	-- Process table recursively
	local result = {}
	local count = 0
	local last_value

	for k, v in pairs(data) do
		count = count + 1
		last_value = M.collapse_on_key(v, key)
		result[k] = last_value
	end

	-- If there's only one entry, return its value
	if count == 1 then
		return last_value
	end

	return result
end

return M
