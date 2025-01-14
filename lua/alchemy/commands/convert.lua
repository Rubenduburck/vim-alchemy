local Config = require("alchemy.config")
local Utils = require("alchemy.utils")
local Rpc = require("alchemy.rpc")
local Ui = require("alchemy.ui")
local Commands = require("alchemy.commands")

local M = {}

function M.convert(args, opts)
	local params = vim.deepcopy(Config.options.commands.convert)

	-- Get selection
	params.selection = Utils.get_visual_selection()
	params.bufnr = vim.api.nvim_get_current_buf()

	-- Parse arguments
	if #args > 0 then
		params.input_encoding = args[1]
	end
	if #args > 1 then
		params.output_encoding = args[2]
	end

	-- Get input encoding and continue with conversion after we have it
	Commands.get_input_encoding(params, function(input_encoding)
		vim.notify("input_encoding: " .. input_encoding)
		params.input_encoding = input_encoding

		-- Convert
		if params.output_encoding == "select" then
			params.output_encoding = Config.options.encodings
		end
		local result = Rpc.convert(params)

		result = Utils.collapse_on_key(result, "output")

		if result == nil then
			vim.notify("No result found")
			return
		elseif type(result) == "string" then
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
end

function M.classify_and_convert(args, opts)
	local params = vim.deepcopy(Config.options.commands.convert)

	-- Get selection
	params.selection = Utils.get_visual_selection()
	params.bufnr = vim.api.nvim_get_current_buf()

	-- Parse arguments
	if #args > 0 then
		params.output_encoding = args[1]
	end

	local result = Rpc.classify_and_convert(params)
	if not result then
		vim.notify("No result found")
		return
	elseif type(result) == "string" then
		-- If result is a string, replace selection with it
		Utils.replace_selection(params.bufnr, params.selection, result)
		return
	else
		-- NOTE: should never get here
		Ui.nested_select(result, function(value)
			-- Replace selection with selected value
			Utils.replace_selection(params.bufnr, params.selection, value)
		end)
	end
end

return M
