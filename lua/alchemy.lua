local M = {}

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
	return {
		text = table.concat(lines, "\n"),
		start_line = line_start - 1,
		start_col = column_start - 1,
		end_line = line_end - 1,
		end_col = column_end,
	}
end

M.jobId = 0

M.AlchClassifyAndConvert = "classify_and_convert"
M.AlchClassify = "classify"
M.AlchConvert = "convert"
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
		vim.api.nvim_err_writeln("Alchemy: cannot start rpc process")
	elseif id == -1 then
		vim.api.nvim_err_writeln("Alchemy: rpc process is not executable")
	else
		M.jobId = id
	end
end

M.configureCommands = function()
	vim.cmd('command! -range -nargs=+ AlchTest :lua require("alchemy").test({output_encoding = <f-args>})')

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

local function visual_replace_rpc(method, params)
	if params.bufnr == nil then
		params.bufnr = vim.api.nvim_get_current_buf()
	end
	if params.selection == nil then
		params.selection = get_visual_selection()
	end

	print(vim.inspect(params))
	local result = vim.rpcrequest(M.jobId, method, params)
	if result then
		local lines = vim.split(result, "\n", { plain = true })
		vim.api.nvim_buf_set_text(
			params.bufnr,
			params.selection.start_line,
			params.selection.start_col,
			params.selection.end_line,
			params.selection.end_col,
			lines
		)
	end
end

local function rpc_replace(method, opts)
	if opts.bufnr == nil then
		opts.bufnr = vim.api.nvim_get_current_buf()
	end

	if opts.selection == nil then
		opts.selection = get_visual_selection()
	end

	-- Check if we have a valid visual selection
	if opts.selection.text ~= nil then
		-- Visual selection mode
		local result = vim.rpcrequest(M.jobId, method, opts)
		if result then
			local lines = vim.split(result, "\n", { plain = true })
			vim.api.nvim_buf_set_text(
				opts.bufnr,
				opts.selection.start_line,
				opts.selection.start_col,
				opts.selection.end_line,
				opts.selection.end_col,
				lines
			)
		end
	else
		-- Current line mode
		local cursor_pos = vim.api.nvim_win_get_cursor(0)
		local current_line = cursor_pos[1] - 1 -- Convert from 1-based to 0-based
		local input = vim.api.nvim_buf_get_lines(opts.bufnr, current_line, current_line + 1, false)[1]
		local result = vim.rpcrequest(M.jobId, opts.command, opts.args, input)
		if result then
			vim.api.nvim_buf_set_lines(opts.bufnr, current_line, current_line + 1, false, { result })
		end
	end
end

-- Define default options for commands
M.defaults = {
	convert = {
		input_encoding = "int",
		output_encoding = "int",
	},
	generate = {
		encoding = "int",
		bytes = "int",
	},
	random = {
		encoding = "int",
		bytes = "int",
	},
	chunk = {
		count = "int",
	},
	rotate = {
		count = 1,
	},
	reverse = {
		depth = 1,
	},
	pad = {
		padding = " ",
	},
}

local function merge_opts(defaults, opts)
	if defaults == nil then
		return opts
	end
	opts = opts or {}
	local result = {}
	for k, v in pairs(defaults) do
		result[k] = opts[k] or v
	end
	return result
end

function M.test(opts)
	print(vim.inspect(opts))
	opts = merge_opts(M.defaults.test, opts)
	opts.command = M.AlchTest
	opts.selection = get_visual_selection()
	opts.bufnr = vim.api.nvim_get_current_buf()
	print(vim.inspect(opts))
	local classifications = M.classify(opts)

	-- Convert the classifications table into a list format for the selector
	local items = {}
	if not classifications then
		return
	end
	for _, classification in ipairs(classifications) do
		table.insert(items, {
			text = string.format(
				"(score %s) (%s) %s",
				classification.score,
				classification.encoding,
				classification.value
			),
			value = classification,
		})
	end

	vim.ui.select(items, {
		prompt = "Select classification:",
		format_item = function(item)
			return item.text
		end,
	}, function(choice)
		if choice then
			-- Do something with the selected classification
			print(vim.inspect(choice.value.encoding))
			opts.input_encoding = choice.value.encoding
			M.convert(opts)
		end
	end)
end

function M.classify(opts)
	return vim.rpcrequest(M.jobId, M.AlchClassify, opts)
end

function M.convert(opts)
	visual_replace_rpc(M.AlchConvert, merge_opts(M.defaults.convert, opts))
end

function M.classify_and_convert(opts)
	visual_replace_rpc(M.AlchClassifyAndConvert, merge_opts(M.defaults.convert, opts))
end

function M.flatten_array(opts)
	visual_replace_rpc(M.AlchFlatten, merge_opts(M.defaults.flatten, opts))
end

function M.chunk_array(opts)
	visual_replace_rpc(M.AlchChunk, merge_opts(M.defaults.chunk, opts))
end

function M.reverse_array(opts)
	visual_replace_rpc(M.AlchReverse, merge_opts(M.defaults.reverse, opts))
end

function M.rotate_array(opts)
	visual_replace_rpc(M.AlchRotate, merge_opts(M.defaults.rotate, opts))
end

function M.generate(opts)
	rpc_replace(M.AlchGenerate, merge_opts(M.defaults.generate, opts))
end

function M.random(opts)
	rpc_replace(M.AlchRandom, merge_opts(M.defaults.random, opts))
end

function M.pad_left(opts)
	visual_replace_rpc(M.AlchPadLeft, merge_opts(M.defaults.pad, opts))
end

function M.pad_right(opts)
	visual_replace_rpc(M.AlchPadRight, merge_opts(M.defaults.pad, opts))
end

function M.hash(opts)
	visual_replace_rpc(M.AlchHash, merge_opts(M.defaults.hash, opts))
end

function M.start()
	local id = M.initRpc()
	M.jobId = id
end

function M.stop()
	vim.rpcnotify(M.jobId, M.AlchStop)
	M.jobId = 0
end

function M.initRpc()
	if M.jobId == 0 then
		local jobid = vim.fn.jobstart({ M.bin }, { rpc = true })
		return jobid
	else
		return M.jobId
	end
end

return M
