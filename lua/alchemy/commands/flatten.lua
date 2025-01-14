local Utils = require("alchemy.utils")
local Rpc = require("alchemy.rpc")

local function flatten(args, opts)
	local params = {}

	-- Get selection
	params.selection = Utils.get_visual_selection()
	params.bufnr = vim.api.nvim_get_current_buf()

	local result = Rpc.flatten_array(params)
	if not result then
		vim.notify("Failed to flatten array", vim.log.levels.ERROR)
		return
	end

	Utils.replace_selection(params.bufnr, params.selection, result)
end
return flatten
