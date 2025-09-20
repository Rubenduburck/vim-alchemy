-- Telescope integration for alchemy
-- Provides enhanced search and selection capabilities

local M = {}

local Core = require("alchemy.core")
local UI = require("alchemy.ui")
local Preview = require("alchemy.preview")

-- Check if telescope is available
local function has_telescope()
	return pcall(require, "telescope")
end

-- Create a telescope picker for conversions
function M.conversion_picker(text_selection)
	text_selection = text_selection or Core.get_text_selection()

	if not has_telescope() then
		-- Fall back to our custom UI
		Preview.show_conversion_explorer(text_selection)
		return
	end

	local pickers = require("telescope.pickers")
	local finders = require("telescope.finders")
	local conf = require("telescope.config").values
	local actions = require("telescope.actions")
	local action_state = require("telescope.actions.state")

	if not text_selection.text or text_selection.text == "" then
		UI.notify("No text to convert", vim.log.levels.WARN)
		return
	end

	-- Build conversion options
	local conversions = {}
	local encodings = { "hex", "int", "base64", "base58", "bin", "utf8", "ascii", "bytes" }

	for _, encoding in ipairs(encodings) do
		table.insert(conversions, {
			display = UI.get_icon_for_type(encoding) .. " Convert to " .. encoding,
			value = encoding,
			type = "convert",
		})
	end

	-- Add hash operations
	local hash_algos = { "sha256", "sha512", "sha3256", "blake2256", "blake2512", "keccak256" }
	for _, algo in ipairs(hash_algos) do
		table.insert(conversions, {
			display = "🔐 Hash with " .. algo:upper(),
			value = algo,
			type = "hash",
		})
	end

	-- Add array operations if applicable
	if text_selection.text:match("%[") or text_selection.text:match(",") then
		table.insert(conversions, {
			display = "📚 Flatten array",
			value = "flatten",
			type = "array",
		})
		table.insert(conversions, {
			display = "🔪 Chunk array",
			value = "chunk",
			type = "array",
		})
		table.insert(conversions, {
			display = "🔄 Rotate array",
			value = "rotate",
			type = "array",
		})
	end

	pickers
		.new({}, {
			prompt_title = "🔄 Alchemy Conversions",
			finder = finders.new_table({
				results = conversions,
				entry_maker = function(entry)
					return {
						value = entry,
						display = entry.display,
						ordinal = entry.display,
					}
				end,
			}),
			sorter = conf.generic_sorter({}),
			attach_mappings = function(prompt_bufnr, map)
				actions.select_default:replace(function()
					actions.close(prompt_bufnr)
					local selection = action_state.get_selected_entry()
					if selection then
						local item = selection.value
						M.execute_conversion(text_selection, item)
					end
				end)

				-- Add preview mapping
				map("i", "<C-p>", function()
					local selection = action_state.get_selected_entry()
					if selection then
						local item = selection.value
						M.preview_conversion(text_selection, item)
					end
				end)

				return true
			end,
		})
		:find()
end

-- Execute a conversion
function M.execute_conversion(text_selection, item)
	local result
	
	if item.type == "convert" then
		result = Core.classify_and_convert(text_selection.text, item.value)
	elseif item.type == "hash" then
		result = Core.execute_cli({ "hash", "-i", "utf8", "-a", item.value, text_selection.text }, false)
	elseif item.type == "array" then
		if item.value == "flatten" then
			result = Core.execute_cli({ "array", "flatten", text_selection.text }, false)
		elseif item.value == "chunk" then
			vim.ui.input({ prompt = "Chunk size: " }, function(size)
				if size and tonumber(size) then
					result = Core.execute_cli({ "array", "chunk", "-c", size, text_selection.text }, false)
					if result then
						Core.replace_text(text_selection, result)
						UI.notify("Array chunked into groups of " .. size, vim.log.levels.INFO)
					end
				end
			end)
			return
		elseif item.value == "rotate" then
			vim.ui.input({ prompt = "Rotation amount: " }, function(amount)
				if amount and tonumber(amount) then
					result = Core.execute_cli({ "array", "rotate", "-r", amount, text_selection.text }, false)
					if result then
						Core.replace_text(text_selection, result)
						UI.notify("Array rotated by " .. amount, vim.log.levels.INFO)
					end
				end
			end)
			return
		end
	end

	if result then
		Core.replace_text(text_selection, result)
		UI.notify("Conversion applied: " .. item.display, vim.log.levels.INFO)
	else
		UI.notify("Conversion failed", vim.log.levels.ERROR)
	end
end

-- Preview a conversion
function M.preview_conversion(text_selection, item)
	local conversion_fn = function()
		if item.type == "convert" then
			return Core.classify_and_convert(text_selection.text, item.value)
		elseif item.type == "hash" then
			return Core.execute_cli({ "hash", "-i", "utf8", "-a", item.value, text_selection.text }, false)
		elseif item.type == "array" then
			if item.value == "flatten" then
				return Core.execute_cli({ "array", "flatten", text_selection.text }, false)
			end
		end
		return nil
	end

	Preview.show_conversion_preview(text_selection, conversion_fn, {
		title = " 👁️ Preview: " .. item.display .. " ",
	})
end

-- Create a classification picker
function M.classification_picker(text_selection)
	text_selection = text_selection or Core.get_text_selection()

	if not has_telescope() then
		-- Fall back to our custom UI
		local ok, classifications = pcall(Core.classify, text_selection.text)
		if ok and classifications then
			Preview.show_classifications(text_selection, classifications)
		end
		return
	end

	if not text_selection.text or text_selection.text == "" then
		UI.notify("No text to classify", vim.log.levels.WARN)
		return
	end

	local ok, classifications = pcall(Core.classify, text_selection.text)
	if not ok or not classifications then
		UI.notify("Classification failed", vim.log.levels.ERROR)
		return
	end

	local pickers = require("telescope.pickers")
	local finders = require("telescope.finders")
	local conf = require("telescope.config").values
	local actions = require("telescope.actions")
	local action_state = require("telescope.actions.state")

	-- Process classifications
	local items = {}
	for _, class in ipairs(classifications) do
		if class.encoding and class.score then
			local confidence = 100 - class.score
			table.insert(items, {
				encoding = class.encoding,
				confidence = confidence,
				display = string.format("%s %s (%.0f%%)", 
					UI.get_icon_for_type(class.encoding), 
					class.encoding, 
					confidence),
			})
		end
	end

	-- Sort by confidence
	table.sort(items, function(a, b)
		return a.confidence > b.confidence
	end)

	pickers
		.new({}, {
			prompt_title = "🎯 Classification Results",
			finder = finders.new_table({
				results = items,
				entry_maker = function(entry)
					return {
						value = entry,
						display = entry.display,
						ordinal = entry.display,
					}
				end,
			}),
			sorter = conf.generic_sorter({}),
			attach_mappings = function(prompt_bufnr, map)
				actions.select_default:replace(function()
					actions.close(prompt_bufnr)
					local selection = action_state.get_selected_entry()
					if selection then
						-- When a classification is selected, show conversion options
						M.conversion_picker(text_selection)
					end
				end)
				return true
			end,
		})
		:find()
end

return M
