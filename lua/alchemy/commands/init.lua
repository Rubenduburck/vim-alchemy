---@class alchemy.commands
local M = {}

local command_opts = require("alchemy.config").commands
local Utils = require("alchemy.utils")
local Rpc = require("alchemy.rpc")
local Config = require("alchemy.config")
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
	vim.notify("classifications: " .. vim.inspect(classifications))

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

function M.get_hashing_algo(params, callback)
	-- If params.input_encoding is not "auto" or "select", just return it directly
	if params.algo ~= "select" then
		callback(params.algo)
		return
	end

	-- Collapse classifications to unique encodings
	local hashers = Config.options.hashers

	-- If there's only one encoding or mode is "auto", use the first one
	if #hashers == 1 then
		callback(hashers[1])
		return
	end

	-- If we're here, we need user selection (params.input_encoding == "select")
	Ui.nested_select(hashers, function(algo)
		callback(algo)
	end)
end

local alchemy_command_tbl = {
	convert = {
		impl = function(args, opts)
			require("alchemy.commands.convert")(args, opts)
		end,
	},
	hash = {
		impl = function(args, opts)
			require("alchemy.commands.hash")(args, opts)
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
	-- pad = {
	--     impl = function(args, opts)
	--         require("alchemy.commands.pad")(args, opts)
	--     end,
	-- },
	-- rotate = {
	--     impl = function(args, opts)
	--         require("alchemy.commands.rotate")(args, opts)
	--     end,
	-- },
	-- reverse = {
	--     impl = function(args, opts)
	--         require("alchemy.commands.reverse")(args, opts)
	--     end,
	-- },
	-- chunk = {
	--     impl = function(args, opts)
	--         require("alchemy.commands.chunk")(args, opts)
	--     end,
	-- },
	-- flatten = {
	--     impl = function(args, opts)
	--         require("alchemy.commands.flatten")(args, opts)
	--     end,
	-- },
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

-- old commands
--

function M.test(opts)
	opts = Utils.merge_opts(command_opts.test, opts)
	opts.selection = Utils.get_visual_selection()
	opts.bufnr = vim.api.nvim_get_current_buf()
	local classifications = Rpc.classify(opts)

	-- Convert the classifications table into a list format for the selector
	local items = {}
	if not classifications then
		return
	end
	for _, classification in ipairs(classifications) do
		table.insert(items, {
			text = string.format(
				"(score %s) (%s) %s",
				classification.score,
				classification.encoding,
				classification.value
			),
			value = classification,
		})
	end

	vim.ui.select(items, {
		prompt = "Select classification:",
		format_item = function(item)
			return item.text
		end,
	}, function(choice)
		if choice then
			-- Do something with the selected classification
			opts.input_encoding = choice.value.encoding
			Rpc.convert(opts)
		end
	end)
end

function M.classify_and_convert(opts)
	print("classify_and_convert opts " .. vim.inspect(opts))
	-- Get selected text
	opts.selection = Utils.get_visual_selection()
	opts.bufnr = vim.api.nvim_get_current_buf()

	-- Classify
	opts = Utils.merge_opts(command_opts.classify, opts)
	local classifications = Rpc.classify(opts)
	if not classifications then
		return
	end

	print("classifications " .. vim.inspect(classifications))
	print("opts " .. vim.inspect(opts))

	-- Handle input encoding selection
	local function process_conversion(input_encoding)
		-- Convert
		opts.input_encoding = input_encoding
		if opts.output_encoding == "select" then
			opts.output_encoding = { "int", "hex", "base64", "utf8", "utf16", "utf32", "ascii" }
		end
		local result = Rpc.convert(opts)

		if not result then
			return
		end
		print("result " .. vim.inspect(result))

		-- Handle the conversion result
		local count = vim.tbl_count(result)

		if count == 0 then
			vim.notify("No results available to process.", vim.log.levels.WARN)
			return
		elseif count == 1 then
			local key, value = next(result)
			if value == nil then
				vim.notify("Error: The single result has a nil value.", vim.log.levels.ERROR)
				return
			end
			Utils.replace_selection(opts.bufnr, opts.selection, value)
		else
			local items = {}
			for encoding, value in pairs(result) do
				if encoding ~= nil and value ~= nil then
					table.insert(items, {
						text = string.format("(%s) %s", encoding, value),
						value = value,
					})
				end
			end

			vim.ui.select(items, {
				prompt = "Output",
				format_item = function(item)
					return item.text
				end,
			}, function(choice)
				if choice then
					Utils.replace_selection(opts.bufnr, opts.selection, choice.value)
				else
					vim.notify("Selection canceled by user.", vim.log.levels.INFO)
				end
			end)
		end
	end

	-- Handle input encoding selection
	if opts.input_encoding == "select" and #classifications > 1 then
		-- Multiple classifications: show selection UI
		local items = {}
		for _, classification in ipairs(classifications) do
			table.insert(items, {
				text = string.format(
					"(score %s) (%s) %s",
					classification.score,
					classification.encoding,
					classification.value
				),
				value = classification,
			})
		end
		vim.ui.select(items, {
			prompt = "Input Encoding",
			format_item = function(item)
				return item.text
			end,
		}, function(choice)
			if choice then
				process_conversion(choice.value.encoding)
			end
		end)
	else
		-- Single classification or automatic selection
		process_conversion(classifications[1].encoding)
	end
end

return M
