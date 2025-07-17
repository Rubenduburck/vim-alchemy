-- Classification explorer mode
-- Shows classifications first, then allows navigation through conversion options

local M = {}

local Core = require("alchemy.core")
local Picker = require("alchemy.picker")

-- Main explorer interface
function M.explore_conversions(text_selection)
	text_selection = text_selection or Core.get_text_selection()

	if not text_selection.text or text_selection.text == "" then
		vim.notify("No text to explore", vim.log.levels.WARN)
		return
	end

	-- First, classify the text
	vim.notify("Classifying text: " .. text_selection.text)

	local ok, classifications = pcall(Core.classify, text_selection.text)
	if not ok or not classifications then
		vim.notify("Failed to classify text: " .. (classifications or "unknown error"), vim.log.levels.ERROR)
		return
	end

	-- Process classifications into a nice format
	local classification_list = {}

	if type(classifications) == "table" and #classifications > 0 then
		for _, item in ipairs(classifications) do
			if type(item) == "table" and item.classification and item.confidence then
				local display_name = string.format("%s (confidence: %d%%)", item.classification, item.confidence)
				table.insert(classification_list, {
					encoding = item.classification:lower(),
					display = display_name,
					confidence = item.confidence,
				})
			end
		end
	end

	-- Sort by confidence (higher is better)
	table.sort(classification_list, function(a, b)
		return a.confidence > b.confidence
	end)

	if #classification_list == 0 then
		vim.notify("No valid classifications found", vim.log.levels.WARN)
		return
	end

	-- Let user pick a classification
	local display_items = {}
	for _, item in ipairs(classification_list) do
		table.insert(display_items, item.display)
	end

	Picker.select_from_list(display_items, {
		prompt = "Select input encoding:",
	}, function(selected_display, idx)
		if not selected_display then
			return
		end

		local selected_classification = classification_list[idx]
		M.show_conversions_for_encoding(text_selection, selected_classification.encoding)
	end)
end

-- Show all available operations for a specific input encoding
function M.show_conversions_for_encoding(text_selection, input_encoding)
	local operations = {
		-- Conversion operations
		{
			type = "convert",
			name = "Convert to hex",
			action = function()
				return Core.convert(text_selection.text, input_encoding, "hex")
			end,
		},
		{
			type = "convert",
			name = "Convert to int",
			action = function()
				return Core.convert(text_selection.text, input_encoding, "int")
			end,
		},
		{
			type = "convert",
			name = "Convert to base64",
			action = function()
				return Core.convert(text_selection.text, input_encoding, "base64")
			end,
		},
		{
			type = "convert",
			name = "Convert to base58",
			action = function()
				return Core.convert(text_selection.text, input_encoding, "base58")
			end,
		},
		{
			type = "convert",
			name = "Convert to binary",
			action = function()
				return Core.convert(text_selection.text, input_encoding, "bin")
			end,
		},
		{
			type = "convert",
			name = "Convert to UTF-8",
			action = function()
				return Core.convert(text_selection.text, input_encoding, "utf8")
			end,
		},
		{
			type = "convert",
			name = "Convert to ASCII",
			action = function()
				return Core.convert(text_selection.text, input_encoding, "ascii")
			end,
		},
		{
			type = "convert",
			name = "Convert to bytes",
			action = function()
				return Core.convert(text_selection.text, input_encoding, "bytes")
			end,
		},

		-- Other operations (work on any format)
		{
			type = "operation",
			name = "Hash with SHA256",
			action = function()
				return M.hash_text(text_selection.text, "sha256")
			end,
		},
		{
			type = "operation",
			name = "Hash with SHA512",
			action = function()
				return M.hash_text(text_selection.text, "sha512")
			end,
		},
		{
			type = "operation",
			name = "Hash with MD5",
			action = function()
				return M.hash_text(text_selection.text, "md5")
			end,
		},
		{
			type = "operation",
			name = "Hash with Blake2",
			action = function()
				return M.hash_text(text_selection.text, "blake2")
			end,
		},
		{
			type = "operation",
			name = "Hash with Keccak256",
			action = function()
				return M.hash_text(text_selection.text, "keccak256")
			end,
		},
	}

	-- Add array-specific operations if the text looks like an array
	if M.is_array_like(text_selection.text) then
		table.insert(operations, {
			type = "array",
			name = "Flatten array",
			action = function()
				return M.flatten_array(text_selection.text)
			end,
		})
		table.insert(operations, {
			type = "array",
			name = "Chunk array",
			action = function()
				return M.chunk_array_interactive(text_selection.text)
			end,
		})
		table.insert(operations, {
			type = "array",
			name = "Rotate array",
			action = function()
				return M.rotate_array_interactive(text_selection.text)
			end,
		})
		table.insert(operations, {
			type = "array",
			name = "Pad left",
			action = function()
				return M.pad_left_interactive(text_selection.text)
			end,
		})
		table.insert(operations, {
			type = "array",
			name = "Pad right",
			action = function()
				return M.pad_right_interactive(text_selection.text)
			end,
		})
	end

	vim.notify(string.format("Showing all operations for %s...", input_encoding))

	-- Create display list
	local operation_names = {}
	for _, op in ipairs(operations) do
		table.insert(operation_names, op.name)
	end

	-- Show operation picker
	Picker.select_from_list(operation_names, {
		prompt = string.format("Operations for %s:", input_encoding),
	}, function(selected_name, idx)
		if not selected_name then
			return
		end

		local selected_op = operations[idx]
		vim.notify(string.format("Executing: %s", selected_op.name))

		local ok, result = pcall(selected_op.action)
		if ok and result and result ~= "" then
			Core.replace_text(text_selection, result)
			vim.notify(string.format("%s: %s", selected_op.name, result))
		else
			vim.notify(string.format("Operation failed: %s", result or "unknown error"), vim.log.levels.ERROR)
		end
	end)
end

-- Quick convert mode - classify and convert in one step
function M.quick_convert(output_encoding)
	local text_selection = Core.get_text_selection()

	if not text_selection.text or text_selection.text == "" then
		vim.notify("No text to convert", vim.log.levels.WARN)
		return
	end

	if not output_encoding then
		-- Let user choose output encoding
		local encodings = { "hex", "int", "base64", "base58", "bin", "utf8", "ascii", "bytes" }

		Picker.select_from_list(encodings, {
			prompt = "Convert to:",
		}, function(selected_encoding)
			if selected_encoding then
				M.quick_convert(selected_encoding)
			end
		end)
		return
	end

	vim.notify(string.format("Converting to %s...", output_encoding))

	local ok, result = pcall(Core.classify_and_convert, text_selection.text, output_encoding)
	if not ok or not result or result == "" then
		vim.notify("Conversion failed: " .. (result or "unknown error"), vim.log.levels.ERROR)
		return
	end

	-- The CLI returns plain text for conversions
	Core.replace_text(text_selection, result)
	vim.notify(string.format("Converted to %s: %s", output_encoding, result))
end

-- Helper functions for additional operations

-- Check if text looks like an array (JSON array, comma-separated, etc.)
function M.is_array_like(text)
	-- Check for JSON array
	if text:match("^%s*%[.*%]%s*$") then
		return true
	end
	-- Check for comma-separated values
	if text:match(",") then
		return true
	end
	-- Check for space-separated numbers/hex values
	if text:match("^[%s%w]+$") and text:match("%s") then
		return true
	end
	return false
end

-- Hash text with specified algorithm
function M.hash_text(text, algorithm)
	local args = { "hash", algorithm, text }
	return Core.execute_cli(args) -- Hash returns plain text
end

-- Flatten array (placeholder - would need CLI support)
function M.flatten_array(text)
	local args = { "flatten", text }
	return Core.execute_cli(args)
end

-- Interactive chunk array
function M.chunk_array_interactive(text)
	vim.ui.input({ prompt = "Chunk size: " }, function(size)
		if size and tonumber(size) then
			local args = { "chunk", size, text }
			local ok, result = pcall(Core.execute_cli, args, false)
			if ok then
				return result
			end
		end
		return nil
	end)
end

-- Interactive rotate array
function M.rotate_array_interactive(text)
	vim.ui.input({ prompt = "Rotate by: " }, function(amount)
		if amount and tonumber(amount) then
			local args = { "rotate", amount, text }
			local ok, result = pcall(Core.execute_cli, args, false)
			if ok then
				return result
			end
		end
		return nil
	end)
end

-- Interactive pad left
function M.pad_left_interactive(text)
	vim.ui.input({ prompt = "Pad to size: " }, function(size)
		if size and tonumber(size) then
			local args = { "pad-left", size, text }
			local ok, result = pcall(Core.execute_cli, args, false)
			if ok then
				return result
			end
		end
		return nil
	end)
end

-- Interactive pad right
function M.pad_right_interactive(text)
	vim.ui.input({ prompt = "Pad to size: " }, function(size)
		if size and tonumber(size) then
			local args = { "pad-right", size, text }
			local ok, result = pcall(Core.execute_cli, args, false)
			if ok then
				return result
			end
		end
		return nil
	end)
end

return M

