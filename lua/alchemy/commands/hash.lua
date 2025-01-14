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
	Commands.get_hashing_algo(params, function(algo)
		params.algo = algo
		print("algo: " .. (algo or "nil"))

		-- Get input encoding
		Commands.get_input_encoding(params, function(encoding)
			params.input_encoding = encoding
			print("encoding: " .. (encoding or "nil"))

			-- Hash
			local result = Rpc.hash(params)

			if not result then
				print("no result")
				return
			end
			result = Utils.collapse_on_key(result, "output")

			if type(result) == "string" then
				-- If result is a string, replace selection with it
				Utils.replace_selection(params.bufnr, params.selection, result)
				return
			else
				-- If result is a table, show a nested select
				Ui.nested_select(result, function(value)
					-- Replace selection with selected value
					Utils.replace_selection(params.bufnr, params.selection, value)
				end)
			end
		end)
	end)
end

return hash
