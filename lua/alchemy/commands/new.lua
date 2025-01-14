local Config = require("alchemy.config")
local Utils = require("alchemy.utils")
local Rpc = require("alchemy.rpc")

local function new(args, opts)
	local params = vim.deepcopy(Config.options.commands.new)

	-- Get selection
	params.selection = Utils.get_visual_selection()
	params.bufnr = vim.api.nvim_get_current_buf()

	-- Parse arguments
	if #args > 0 then
		params.encoding = args[1]
	end
	if #args > 1 then
		params.bytes = tonumber(args[2])
	end

	local result = Rpc.generate(params)
	if not result then
		vim.notify("Failed to generate new data", vim.log.levels.ERROR)
		return
	end
	Utils.replace_selection(params.bufnr, params.selection, result)
end

return new
