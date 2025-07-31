---@diagnostic disable: missing-fields
local M = {}

function M.defaults()
	local plugin_dir = debug.getinfo(1, "S").source:sub(2):match("(.*/)")
	if plugin_dir then
		plugin_dir = vim.fn.fnamemodify(plugin_dir .. "/../../../", ":p")  -- Go up to plugin root and normalize
	end
	
	-- Try to find binary in various locations
	local possible_paths = {
		plugin_dir and (plugin_dir .. "bin/alchemy") or nil,  -- Local installation
		vim.fn.expand("~/.local/bin/alchemy"),                -- User local
		"alchemy"                                             -- In PATH
	}
	
	local bin = "alchemy"  -- Default fallback
	for _, path in ipairs(possible_paths) do
		if path and vim.fn.executable(path) == 1 then
			bin = path
			break
		end
	end
	---@class AlchemyConfig
	local defaults = {
		cli = {
			bin = bin,
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
		input_encodings = {
			"int",
			"hex",
			"bin",
			"base58",
			"base64",
			"utf8",
			"utf16",
			"ascii",
		},
		output_encodings = {
			"int",
			"hex",
			"bin",
			"bytes",
			"[int]",
			"base58",
			"base64",
			"utf8",
			"utf16",
			"ascii",
		},
		commands = {
			classify = {
				input_encoding = "auto",
			},
			convert = {
				input_encoding = "select",
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
				chunk_count = 1,
			},
			rotate = {
				rotation = 1,
			},
			pad = {
				padding = 32,
			},
		},
		ui = {
			float_opts = {
				relative = "cursor",
				row = 1,
				col = 0,
				style = "minimal",
				border = "none", -- removed borders
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

	-- Check if CLI binary exists
	local cli_bin = M.options.cli.bin
	if vim.fn.executable(cli_bin) == 0 then
		vim.notify("Alchemy: CLI binary not found at " .. cli_bin, vim.log.levels.ERROR)
		M._running = false
	else
		M._running = true
	end
end

return M
