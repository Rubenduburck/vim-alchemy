local M = {}

M.convertJobId = 0

local function get_visual_selection()
	local line_start, column_start = vim.fn.getpos("'<")[2], vim.fn.getpos("'<")[3]
	local line_end, column_end = vim.fn.getpos("'>")[2], vim.fn.getpos("'>")[3]
	local lines = vim.fn.getline(line_start, line_end)
	if #lines == 0 then
		return ""
	end
	lines[#lines] = lines[#lines]:sub(1, column_end + 1 - (vim.o.selection == "inclusive" and 1 or 2))
	lines[1] = lines[1]:sub(column_start)
	return table.concat(lines, "\n")
end

M.Alch = "classify_and_convert"
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
M.bin = M.dir .. "../target/release/vim-alchemy"

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
		M.configureCommands()
	end
end

M.configureCommands = function()
	vim.cmd('command! -range -nargs=+ Alch :lua require("alchemy").classify_and_convert(<f-args>)')
	vim.cmd('command! -range AlchFlatten :lua require("alchemy").flatten_array()')
	vim.cmd('command! -range -nargs=+ AlchChunk :lua require("alchemy").chunk_array(<f-args>)')
	vim.cmd('command! -range -nargs=* AlchReverse :lua require("alchemy").reverse_array()')
	vim.cmd('command! -range -nargs=+ AlchRotate :lua require("alchemy").rotate_array(<f-args>)')
	vim.cmd('command! -nargs=+ AlchGenerate :lua require("alchemy").generate(<f-args>)')
	vim.cmd('command! -nargs=+ AlchRandom :lua require("alchemy").random(<f-args>)')
	vim.cmd('command! -range -nargs=+ AlchPadLeft :lua require("alchemy").pad_left(<f-args>)')
	vim.cmd('command! -range -nargs=+ AlchPadRight :lua require("alchemy").pad_right(<f-args>)')
    vim.cmd('command! -range -nargs=* AlchHash :lua require("alchemy").hash(<f-args>)')
	vim.cmd('command! AlchStart :lua require("alchemy").start()')
	vim.cmd('command! AlchStop :lua require("alchemy").stop()')
end

function M.classify_and_convert(...)
	local encoding = select(1, ...) or "int"
	local input = get_visual_selection()
	vim.rpcnotify(M.convertJobId, M.Alch, encoding, input)
end

function M.flatten_array()
	local input = get_visual_selection()
	vim.rpcnotify(M.convertJobId, M.AlchFlatten, input)
end

function M.chunk_array(...)
	local chunk_count = select(1, ...) or "int"
	local input = get_visual_selection()
	vim.rpcnotify(M.convertJobId, M.AlchChunk, chunk_count, input)
end

function M.reverse_array(...)
	local depth = select(1, ...) or "1"
	local input = get_visual_selection()
	vim.rpcnotify(M.convertJobId, M.AlchReverse, depth, input)
end

function M.rotate_array(...)
	local rotation = select(1, ...) or "1"
	local input = get_visual_selection()
	vim.rpcnotify(M.convertJobId, M.AlchRotate, rotation, input)
end

function M.generate(...)
	local encoding = select(1, ...) or "int"
	local bytes = select(2, ...) or "int"
	vim.rpcnotify(M.convertJobId, M.AlchGenerate, encoding, bytes)
end

function M.random(...)
	local encoding = select(1, ...) or "int"
	local bytes = select(2, ...) or "int"
	vim.rpcnotify(M.convertJobId, M.AlchRandom, encoding, bytes)
end

function M.pad_left(...)
	local padding = select(1, ...) or " "
	local input = get_visual_selection()
	vim.rpcnotify(M.convertJobId, M.AlchPadLeft, padding, input)
end

function M.pad_right(...)
	local padding = select(1, ...) or " "
	local input = get_visual_selection()
	vim.rpcnotify(M.convertJobId, M.AlchPadRight, padding, input)
end

function M.hash(...)
    local algorithm = select(1, ...) or "sha256"
    local input = get_visual_selection()
    vim.rpcnotify(M.convertJobId, M.AlchHash, algorithm, input)
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
