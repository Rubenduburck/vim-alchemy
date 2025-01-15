local Utils = require("alchemy.utils")
local Rpc = require("alchemy.rpc")
local Config = require("alchemy.config")

-- TODO: fix rotation to handle padding
local function rotate(args, opts)
	local params = Config.options.commands.rotate

	if #args > 0 then
		params.rotation = tonumber(args[1])
	end

	-- Get selection
	params.selection = Utils.get_visual_selection()
	params.bufnr = vim.api.nvim_get_current_buf()

	local result = Rpc.rotate_array(params)
	if not result then
		vim.notify("Failed to rotate array", vim.log.levels.ERROR)
		return
	end

	Utils.replace_selection(params.bufnr, params.selection, result)
end

return rotate
