---@diagnostic disable: missing-fields
local M = {}

function M.defaults()
	local dir = debug.getinfo(1, "S").source:sub(2):match("(.*/)")
	---@class AlchemyConfig
	local defaults = {
		rpc = {
			bin = dir .. "../../../target/debug/vim-alchemy",
		},
		hashers = {
			"md5",
			"sha1",
			"sha256",
			"sha512",
			"keccak256",
			"keccak512",
			"blake2b",
		},
		encodings = {
			"int",
			"hex",
			"base64",
			"bytes",
			"utf8",
			"utf16",
			"ascii",
		},
		commands = {
			classify = {
				input_encoding = "auto",
			},
			convert = {
				output_encoding = "select",
			},
            hash = {
                input_encoding = "select",
                algo = "select",
            },
			new = {
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
				padding = 1,
			},
		},
	}
	return defaults
end

---@type AlchemyConfig
M.options = {}

M._running = false
function M.is_running()
	return M._running
end

function M.setup(options)
	options = options or {}
	M.options = vim.tbl_deep_extend("force", M.defaults(), options)

	require("alchemy.commands").setup()

	local Rpc = require("alchemy.rpc")
	local resp = Rpc.setup()
	if not resp then
		vim.notify("Alchemy: Could not start RPC server", vim.log.levels.ERROR)
	else
		vim.notify("Alchemy: RPC server started", vim.log.levels.DEBUG)
	end
	M._running = true
end

return M
