local Utils = require("alchemy.utils")
local Rpc = require("alchemy.rpc")
local Config = require("alchemy.config")

local M = {}

function M.pad_left(args, opts)
	local params = Config.options.commands.pad

    if #args > 0 then
        params.padding = tonumber(args[1])
    end

	-- Get selection
	params.selection = Utils.get_visual_selection()
	params.bufnr = vim.api.nvim_get_current_buf()

	local result = Rpc.pad_left(params)
	if not result then
		print("no result")
		return
	end
	print("result: " .. vim.inspect(result))

	Utils.replace_selection(params.bufnr, params.selection, result)
end

function M.pad_right(args, opts)
	local params = Config.options.commands.pad

    if #args > 0 then
        params.padding = tonumber(args[1])
    end

	-- Get selection
	params.selection = Utils.get_visual_selection()
	params.bufnr = vim.api.nvim_get_current_buf()

	local result = Rpc.pad_right(params)
	if not result then
		print("no result")
		return
	end
	print("result: " .. vim.inspect(result))

	Utils.replace_selection(params.bufnr, params.selection, result)
end

return M
