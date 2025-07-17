local M = {}

local Config = require("alchemy.config")

-- Helper function to execute CLI command and parse JSON response
local function execute_cli(cmd_args)
	local cmd = vim.list_extend({ Config.options.cli.bin or "vim-alchemy" }, cmd_args)
	local result = vim.fn.systemlist(cmd)
	local output = table.concat(result, "\n")
	
	-- Check for errors
	if vim.v.shell_error ~= 0 then
		error("CLI command failed: " .. output)
	end
	
	-- Parse JSON response
	local ok, parsed = pcall(vim.json.decode, output)
	if not ok then
		error("Failed to parse CLI response: " .. output)
	end
	
	return parsed
end

-- Helper to format array arguments
local function format_array_arg(values)
	return table.concat(values, ",")
end

function M.classify_and_convert(opts)
	local args = {
		"classify-and-convert",
		"-o", format_array_arg(opts.output_encoding),
		opts.selection.text
	}
	return execute_cli(args)
end

function M.classify(opts)
	local args = {
		"classify",
		opts.selection.text
	}
	return execute_cli(args)
end

function M.convert(opts)
	local args = { "convert" }
	
	-- Only add input encoding if specified
	if opts.input_encoding and #opts.input_encoding > 0 then
		table.insert(args, "-i")
		table.insert(args, format_array_arg(opts.input_encoding))
	end
	
	table.insert(args, "-o")
	table.insert(args, format_array_arg(opts.output_encoding))
	table.insert(args, opts.selection.text)
	
	return execute_cli(args)
end

function M.flatten_array(opts)
	local args = {
		"flatten-array",
		opts.selection.text
	}
	return execute_cli(args)
end

function M.chunk_array(opts)
	local args = {
		"chunk-array",
		"-c", tostring(opts.chunk_count),
		opts.selection.text
	}
	return execute_cli(args)
end

function M.reverse_array(opts)
	local args = {
		"reverse-array",
		"-d", tostring(opts.depth),
		opts.selection.text
	}
	return execute_cli(args)
end

function M.rotate_array(opts)
	local args = {
		"rotate-array",
		"-r", tostring(opts.rotation),
		opts.selection.text
	}
	return execute_cli(args)
end

function M.generate(opts)
	local args = {
		"generate",
		"-e", opts.encoding,
		"-b", tostring(opts.bytes)
	}
	return execute_cli(args)
end

function M.random(opts)
	local args = {
		"random",
		"-e", opts.encoding,
		"-b", tostring(opts.bytes)
	}
	return execute_cli(args)
end

function M.pad_left(opts)
	local args = {
		"pad-left",
		"-p", tostring(opts.padding),
		opts.selection.text
	}
	return execute_cli(args)
end

function M.pad_right(opts)
	local args = {
		"pad-right",
		"-p", tostring(opts.padding),
		opts.selection.text
	}
	return execute_cli(args)
end

function M.classify_and_hash(opts)
	local args = {
		"classify-and-hash",
		"-a", format_array_arg(opts.algo),
		opts.selection.text
	}
	return execute_cli(args)
end

function M.hash(opts)
	local args = {
		"hash",
		"-a", format_array_arg(opts.algo),
		"-i", format_array_arg(opts.input_encoding),
		opts.selection.text
	}
	return execute_cli(args)
end

-- No-op functions for compatibility
function M.stopRpc()
	-- CLI doesn't need to be stopped
end

function M.initRpc()
	-- CLI doesn't need initialization
	return 1
end

function M.setup()
	-- CLI doesn't need setup
	-- Config is handled by the Lua plugin
	return true
end

return M