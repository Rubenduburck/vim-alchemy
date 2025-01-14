local M = {}

local Config = require("alchemy.config")

M.jobId = 0

M.AlchClassifyAndConvert = "classify_and_convert"
M.AlchClassify = "classify"
M.AlchConvert = "convert"
M.AlchFlatten = "flatten_array"
M.AlchChunk = "chunk_array"
M.AlchReverse = "reverse_array"
M.AlchRotate = "rotate_array"
M.AlchGenerate = "generate"
M.AlchRandom = "random"
M.AlchPadLeft = "pad_left"
M.AlchPadRight = "pad_right"
M.AlchStart = "start"
M.AlchStop = "stop"
M.AlchHash = "hash"
M.AlchSetup = "setup"

function M.classify_and_convert(opts)
	return vim.rpcrequest(M.jobId, M.AlchClassifyAndConvert, opts)
end

function M.classify(opts)
	return vim.rpcrequest(M.jobId, M.AlchClassify, opts)
end

function M.convert(opts)
	return vim.rpcrequest(M.jobId, M.AlchConvert, opts)
end

function M.flatten_array(opts)
	return vim.rpcrequest(M.jobId, M.AlchFlatten, opts)
end

function M.chunk_array(opts)
	return vim.rpcrequest(M.jobId, M.AlchChunk, opts)
end

function M.reverse_array(opts)
	return vim.rpcrequest(M.jobId, M.AlchReverse, opts)
end

function M.rotate_array(opts)
	return vim.rpcrequest(M.jobId, M.AlchRotate, opts)
end

function M.generate(opts)
	return vim.rpcrequest(M.jobId, M.AlchGenerate, opts)
end

function M.random(opts)
	return vim.rpcrequest(M.jobId, M.AlchRandom, opts)
end

function M.pad_left(opts)
	return vim.rpcrequest(M.jobId, M.AlchPadLeft, opts)
end

function M.pad_right(opts)
	return vim.rpcrequest(M.jobId, M.AlchPadRight, opts)
end

function M.hash(opts)
	return vim.rpcrequest(M.jobId, M.AlchHash, opts)
end

function M.stopRpc()
	vim.rpcrequest(M.jobId, "stop")
	M.jobId = 0
end

function M.initRpc()
	if M.jobId == 0 then
		local jobid = vim.fn.jobstart({ Config.options.rpc.bin }, { rpc = true })
		return jobid
	else
		return M.jobId
	end
end

function M.setup()
	M.jobId = M.initRpc()
	local config = {
		config = {
			classifier = {
				available_encodings = Config.options.encodings,
			},
		},
	}
	return vim.rpcrequest(M.jobId, M.AlchSetup, config)
end

return M
