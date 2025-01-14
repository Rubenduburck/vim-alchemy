local Utils = require("alchemy.utils")
local Rpc = require("alchemy.rpc")
local Config = require("alchemy.config")

-- TODO: fix reverse to handle padding
local function reverse(args, opts)
	local params = Config.options.commands.reverse

	if #args > 0 then
		params.depth = tonumber(args[1])
	end

	-- Get selection
	params.selection = Utils.get_visual_selection()
	params.bufnr = vim.api.nvim_get_current_buf()

	local result = Rpc.reverse_array(params)
	if not result then
		print("no result")
		return
	end
	print("result: " .. vim.inspect(result))

	Utils.replace_selection(params.bufnr, params.selection, result)
end

return reverse
