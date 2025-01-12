local M = {}

M.convertJobId = 0

local function get_visual_selection()
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
	return table.concat(lines, "\n"), line_start - 1, column_start - 1, line_end - 1, column_end
end

M.AlchClassifyAndConvert = "classify_and_convert"
M.AlchFlatten = "flatten_array"
M.AlchChunk = "chunk_array"
M.AlchReverse = "reverse_array"
M.AlchRotate = "rotate_array"
M.AlchGenerate = "generate"
M.AlchRandom = "random"
M.AlchPadLeft = "pad_left"
M.AlchPadRight = "pad_right"
M.AlchStart = "start"
M.AlchStop = "stop"
M.AlchHash = "hash"

-- TODO: what is the proper way to do this?
M.dir = debug.getinfo(1, "S").source:sub(2):match("(.*/)")
M.bin = M.dir .. "../target/debug/vim-alchemy"
print("bin: " .. M.bin)

M.setup = function(opts)
	M.configureCommands()
	M.connect()
end

M.connect = function()
	local id = M.initRpc()

	if id == 0 then
		vim.api.nvim_err_writeln("convert: cannot start rpc process")
	elseif id == -1 then
		vim.api.nvim_err_writeln("convert: rpc process is not executable")
	else
		M.convertJobId = id
	end
end

M.configureCommands = function()
	vim.cmd('command! -range -nargs=+ Alch :lua require("alchemy").classify_and_convert(<f-args>)')
	vim.cmd('command! -range AlchFlatten :lua require("alchemy").flatten_array()')
	vim.cmd('command! -range -nargs=+ AlchChunk :lua require("alchemy").chunk_array(<f-args>)')
	vim.cmd('command! -range -nargs=* AlchReverse :lua require("alchemy").reverse_array()')
	vim.cmd('command! -range -nargs=+ AlchRotate :lua require("alchemy").rotate_array(<f-args>)')
	vim.cmd('command! -range -nargs=+ AlchGenerate :lua require("alchemy").generate(<f-args>)')
	vim.cmd('command! -range -nargs=+ AlchRandom :lua require("alchemy").random(<f-args>)')
	vim.cmd('command! -range -nargs=+ AlchPadLeft :lua require("alchemy").pad_left(<f-args>)')
	vim.cmd('command! -range -nargs=+ AlchPadRight :lua require("alchemy").pad_right(<f-args>)')
	vim.cmd('command! -range -nargs=* AlchHash :lua require("alchemy").hash(<f-args>)')
	vim.cmd('command! AlchStart :lua require("alchemy").start()')
	vim.cmd('command! AlchStop :lua require("alchemy").stop()')
end

local function visual_replace_rpc(command, ...)
	local bufnr = vim.api.nvim_get_current_buf()
	local input, start_line, start_col, end_line, end_col = get_visual_selection()
	if start_line == nil or start_col == nil or end_line == nil or end_col == nil then
		return
	end

	local args = { ... }
	table.insert(args, input)
	local result = vim.rpcrequest(M.convertJobId, command, unpack(args))
	if result then
		local lines = vim.split(result, "\n", { plain = true })
		vim.api.nvim_buf_set_text(bufnr, start_line, start_col, end_line, end_col, lines)
	end
end

local function insert_at_cursor_rpc(command, ...)
	local bufnr = vim.api.nvim_get_current_buf()
	local input = vim.api.nvim_get_current_line()
	local args = { ... }
	table.insert(args, input)
	local result = vim.rpcrequest(M.convertJobId, command, unpack(args))
	if result then
		vim.api.nvim_buf_set_lines(
			bufnr,
			vim.api.nvim_win_get_cursor(0)[1] - 1,
			vim.api.nvim_win_get_cursor(0)[1],
			false,
			{ result }
		)
	end
end

local function rpc_replace(command, ...)
	local bufnr = vim.api.nvim_get_current_buf()
	local input, start_line, start_col, end_line, end_col = get_visual_selection()
	local args = { ... }

	-- Check if we have a valid visual selection
	if start_line ~= nil and start_col ~= nil and end_line ~= nil and end_col ~= nil then
		-- Visual selection mode
		table.insert(args, input)
		local result = vim.rpcrequest(M.convertJobId, command, unpack(args))
		if result then
			local lines = vim.split(result, "\n", { plain = true })
			vim.api.nvim_buf_set_text(bufnr, start_line, start_col, end_line, end_col, lines)
		end
	else
		-- Current line mode
		local cursor_pos = vim.api.nvim_win_get_cursor(0)
		local current_line = cursor_pos[1] - 1  -- Convert from 1-based to 0-based
		input = vim.api.nvim_buf_get_lines(bufnr, current_line, current_line + 1, false)[1]
		table.insert(args, input)
		local result = vim.rpcrequest(M.convertJobId, command, unpack(args))
		if result then
			vim.api.nvim_buf_set_lines(
				bufnr,
				current_line,
				current_line + 1,
				false,
				{ result }
			)
		end
	end
end

function M.classify_and_convert(...)
	local encoding = select(1, ...) or "int"
	visual_replace_rpc(M.AlchClassifyAndConvert, encoding)
end

function M.flatten_array()
	visual_replace_rpc(M.AlchFlatten)
end

function M.chunk_array(...)
	local chunk_count = select(1, ...) or "int"
	visual_replace_rpc(M.AlchChunk, chunk_count)
end

function M.reverse_array(...)
	local depth = select(1, ...) or "1"
	visual_replace_rpc(M.AlchReverse, depth)
end

function M.rotate_array(...)
	local rotation = select(1, ...) or "1"
	visual_replace_rpc(M.AlchRotate, rotation)
end

function M.generate(...)
	local encoding = select(1, ...) or "int"
	local bytes = select(2, ...) or "int"
	rpc_replace(M.AlchGenerate, encoding, bytes)
end

function M.random(...)
	local encoding = select(1, ...) or "int"
	local bytes = select(2, ...) or "int"
	rpc_replace(M.AlchRandom, encoding, bytes)
end

function M.pad_left(...)
	local padding = select(1, ...) or " "
	local input = get_visual_selection()
    visual_replace_rpc(M.AlchPadLeft, padding, input)
end

function M.pad_right(...)
	local padding = select(1, ...) or " "
	local input = get_visual_selection()
    visual_replace_rpc(M.AlchPadRight, padding, input)
end

function M.hash(...)
	local algorithm = select(1, ...) or "keccak256"
	local input = get_visual_selection()
    visual_replace_rpc(M.AlchHash, algorithm, input)
end

function M.start()
	local id = M.initRpc()
	M.convertJobId = id
end

function M.stop()
	vim.rpcnotify(M.convertJobId, M.AlchStop)
	M.convertJobId = 0
end

function M.initRpc()
	if M.convertJobId == 0 then
		local jobid = vim.fn.jobstart({ M.bin }, { rpc = true })
		return jobid
	else
		return M.convertJobId
	end
end

return M
