local Config = require("alchemy.config")
local Utils = require("alchemy.utils")
local Rpc = require("alchemy.rpc")
local Ui = require("alchemy.ui")
local Commands = require("alchemy.commands")

local function hash(args, opts)
	local params = vim.deepcopy(Config.options.commands.hash)

	-- Get selection
	params.selection = Utils.get_visual_selection()
	params.bufnr = vim.api.nvim_get_current_buf()

	-- Parse arguments
	if #args > 0 then
		params.input_encoding = args[1]
	end
	if #args > 1 then
		params.algo = args[2]
	end

	print("algo: " .. (params.algo or "nil"))
	print("input_encoding: " .. (params.input_encoding or "nil"))

	-- Get input encoding and continue with conversion after we have it
	-- Get input encoding
	Commands.get_input_encoding(params, function(encoding)
		params.input_encoding = encoding
		print("encoding: " .. (encoding or "nil"))

		if not params.algo or params.algo == "select" then
			params.algo = Config.options.hashers
		end

		local hashes = Rpc.hash(params)
		hashes = Utils.collapse_on_key(hashes, "output")
		print("hashes: " .. vim.inspect(hashes))

		if not hashes then
			vim.notify("No hashes found")
			return
		elseif type(hashes) == "string" then
			Utils.replace_selection(params.bufnr, params.selection, hashes)
		else
			Ui.nested_select(hashes, function(selection)
				Utils.replace_selection(params.bufnr, params.selection, selection)
			end)
		end
	end)
end

return hash
