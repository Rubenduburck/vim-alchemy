--@toc alchemy.contents

--@mod alchemy.intro Introduction
--@brief [[
--This plugin is a helper to convert between different formats of data.
--@brief ]]
--
--@mod alchemy
--
--@brief [[
--
--Commands:
--TODO: add commands

local M = {}

local Config = require("alchemy.config")
local Commands = require("alchemy.commands")

---@params opts? AlchemyConfig
function M.setup(opts)
	require("alchemy.config").setup(opts)
end

function M.start()
	local id = M.initRpc()
	M.jobId = id
end

function M.stop()
	vim.rpcnotify(M.jobId, M.AlchStop)
	M.jobId = 0
end

function M.initRpc()
	if M.jobId == 0 then
		local jobid = vim.fn.jobstart({ M.bin }, { rpc = true })
		return jobid
	else
		return M.jobId
	end
end

return M
