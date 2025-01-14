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
--
-- ':Alch classify_and_convert <to>' - auto classify and convert data
-- ':Alch classify_and_hash <algo>' - auto classify and hash data
-- ':Alch hash <from> <algo>' - hash data
-- ':Alch convert <from> <to>' - convert data from one encoding to another
-- ':Alch flatten <from>' - flatten data
-- ':Alch pad_left <from> <to>' - pad data to a specific length from the left
-- ':Alch pad_right <from> <to>' - pad data to a specific length from the right
-- ':Alch random <to>' - generate random data for some encoding
-- ':Alch reverse <from>' - reverse data
-- ':Alch rotate <from> <count>' - rotate data by a specific count

local M = {}

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
