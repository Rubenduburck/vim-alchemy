local Config = require("alchemy")
local Utils = require("alchemy.utils")
local Rpc = require("alchemy.rpc")

local function chunk(args, opts)
	local params = vim.deepcopy(Config.options.commands.chunk)

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

	local result = Rpc.chunk_array(params)
	if not result then
		vim.notify("Failed to generate new data", vim.log.levels.ERROR)
		return
	end
	Utils.replace_selection(params.bufnr, params.selection, result)
end

return chunk
