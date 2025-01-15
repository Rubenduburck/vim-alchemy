---@class alchemy.commands
local M = {}

local Utils = require("alchemy.utils")
local Rpc = require("alchemy.rpc")
local Ui = require("alchemy.ui")

local alchemy_cmd_name = "Alch"

function M.get_input_encoding(params, callback)
	-- If params.input_encoding is not "auto" or "select", just return it directly
	if params.input_encoding ~= "auto" and params.input_encoding ~= "select" then
		callback(params.input_encoding)
		return
	end

	-- Get classifications from RPC
	local classifications = Rpc.classify(params)
	if not classifications then
		callback(nil)
		return
	end

	-- Collapse classifications to unique encodings
	classifications = Utils.collapse_on_key(classifications, "encoding")

	-- If there's only one encoding or mode is "auto", use the first one
	if #classifications == 1 or params.input_encoding == "auto" then
		callback(classifications[1])
		return
	end

	-- If we're here, we need user selection (params.input_encoding == "select")
	Ui.nested_select(classifications, function(encoding)
		callback(encoding)
	end)
end

local alchemy_command_tbl = {
	classify_and_convert = {
		impl = function(args, opts)
			require("alchemy.commands.convert").classify_and_convert(args, opts)
		end,
	},
	classify_and_hash = {
		impl = function(args, opts)
			require("alchemy.commands.hash").classify_and_hash(args, opts)
		end,
	},
	convert = {
		impl = function(args, opts)
			require("alchemy.commands.convert").convert(args, opts)
		end,
	},
	hash = {
		impl = function(args, opts)
			require("alchemy.commands.hash").hash(args, opts)
		end,
	},
	random = {
		impl = function(args, opts)
			require("alchemy.commands.random")(args, opts)
		end,
	},
	new = {
		impl = function(args, opts)
			require("alchemy.commands.new")(args, opts)
		end,
	},
	pad_left = {
		impl = function(args, opts)
			require("alchemy.commands.pad").pad_left(args, opts)
		end,
	},
	pad_right = {
		impl = function(args, opts)
			require("alchemy.commands.pad").pad_right(args, opts)
		end,
	},
	rotate = {
		impl = function(args, opts)
			require("alchemy.commands.rotate")(args, opts)
		end,
	},
	reverse = {
		impl = function(args, opts)
			require("alchemy.commands.reverse")(args, opts)
		end,
	},
	chunk = {
		impl = function(args, opts)
			require("alchemy.commands.chunk")(args, opts)
		end,
	},
	flatten = {
		impl = function(args, opts)
			require("alchemy.commands.flatten")(args, opts)
		end,
	},
}

local function run_command(opts)
	local fargs = opts.fargs
	local cmd = fargs[1]
	local args = #fargs > 1 and vim.list_slice(fargs, 2, #fargs) or {}
	local command = alchemy_command_tbl[cmd]
	if type(command) ~= "table" or type(command.impl) ~= "function" then
		vim.notify("Alch: Unknown subcommand: " .. cmd, vim.log.levels.ERROR)
		return
	end
	command.impl(args, opts)
end

local function alch(opts)
	run_command(opts)
end

---@generic K, V
---@param predicate fun(V):boolean
---@param tbl table<K, V>
---@return K[]
local function tbl_keys_by_value_filter(predicate, tbl)
	local ret = {}
	for k, v in pairs(tbl) do
		if predicate(v) then
			ret[k] = v
		end
	end
	return vim.tbl_keys(ret)
end

function M.create_alchemy_command()
	vim.api.nvim_create_user_command(alchemy_cmd_name, alch, {
		nargs = "+",
		range = true,
		bang = true,
		desc = "Alchemy commands",
		complete = function(arg_lead, cmdline, _)
			local commands = cmdline:match("^['<,'>]*" .. alchemy_cmd_name .. "!") ~= nil
					-- bang! NOTE: what does bang mean
					and tbl_keys_by_value_filter(function(command)
						return command.bang == true
					end, alchemy_command_tbl)
				or vim.tbl_keys(alchemy_command_tbl)
			local subcmd, subcmd_arg_lead = cmdline:match("^['<,'>]*" .. alchemy_cmd_name .. "[!]*%s(%S+)%s(.*)$")
			if subcmd and subcmd_arg_lead and alchemy_command_tbl[subcmd] and alchemy_command_tbl[subcmd].complete then
				return alchemy_command_tbl[subcmd].complete(subcmd_arg_lead)
			end
			if cmdline:match("^['<,'>]*" .. alchemy_cmd_name .. "[!]*%s+%w*$") then
				return vim.tbl_filter(function(command)
					return command:find(arg_lead) ~= nil
				end, commands)
			end
		end,
	})
end

function M.setup()
	M.create_alchemy_command()
end

return M
