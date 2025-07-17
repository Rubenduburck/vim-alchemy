-- Modern picker interface using vim.ui.select for better UX
-- Provides dropdown menus for selection with nice formatting

local M = {}

-- Format a key-value pair for display
local function format_item(key, value)
    if type(value) == "table" then
        return string.format("%s →", key)
    else
        return string.format("%s: %s", key, tostring(value))
    end
end

-- Create a flat list of items from nested table structure
local function flatten_choices(data, prefix)
    prefix = prefix or ""
    local choices = {}
    
    for key, value in pairs(data) do
        local display_key = prefix == "" and key or (prefix .. " → " .. key)
        
        if type(value) == "table" then
            -- For nested tables, add both the parent option and flatten children
            table.insert(choices, {
                key = display_key,
                value = value,
                display = format_item(key, value),
                is_nested = true
            })
            
            -- Also add flattened children
            local child_choices = flatten_choices(value, display_key)
            for _, child in ipairs(child_choices) do
                table.insert(choices, child)
            end
        else
            table.insert(choices, {
                key = display_key,
                value = value,
                display = format_item(key, value),
                is_nested = false
            })
        end
    end
    
    return choices
end

-- Modern picker using vim.ui.select with nice formatting
function M.select_from_results(data, opts, callback)
    opts = opts or {}
    
    if not data or vim.tbl_isempty(data) then
        vim.notify("No results to select from", vim.log.levels.WARN)
        return
    end
    
    -- If there's only one result and it's a simple value, return it directly
    local count = 0
    local single_key, single_value
    for k, v in pairs(data) do
        count = count + 1
        single_key, single_value = k, v
        if count > 1 then break end
    end
    
    if count == 1 and type(single_value) ~= "table" then
        callback(single_value, single_key)
        return
    end
    
    -- Create choices for picker
    local choices = flatten_choices(data)
    
    -- Filter out nested entries if we have leaf values (cleaner interface)
    local leaf_choices = vim.tbl_filter(function(choice)
        return not choice.is_nested
    end, choices)
    
    local final_choices = #leaf_choices > 0 and leaf_choices or choices
    
    -- Prepare items for vim.ui.select
    local items = {}
    for _, choice in ipairs(final_choices) do
        table.insert(items, choice.display)
    end
    
    local select_opts = {
        prompt = opts.prompt or "Select conversion:",
        format_item = function(item)
            return item
        end,
        kind = "alchemy_conversion"
    }
    
    vim.ui.select(items, select_opts, function(selected_item, idx)
        if not selected_item or not idx then
            return  -- User cancelled
        end
        
        local selected_choice = final_choices[idx]
        if selected_choice.is_nested then
            -- User selected a nested category, show its contents
            M.select_from_results(selected_choice.value, opts, callback)
        else
            -- User selected a final value
            callback(selected_choice.value, selected_choice.key)
        end
    end)
end

-- Simple picker for choosing from a list of strings
function M.select_from_list(items, opts, callback)
    opts = opts or {}
    
    if not items or #items == 0 then
        vim.notify("No items to select from", vim.log.levels.WARN)
        return
    end
    
    if #items == 1 then
        callback(items[1], 1)
        return
    end
    
    local select_opts = {
        prompt = opts.prompt or "Select option:",
        format_item = function(item)
            return tostring(item)
        end,
        kind = "alchemy_list"
    }
    
    vim.ui.select(items, select_opts, function(selected_item, idx)
        if selected_item then
            callback(selected_item, idx)
        end
    end)
end

-- Auto-complete style picker - shows options and replaces text on selection
function M.autocomplete_replace(selection, conversions, opts)
    opts = opts or {}
    
    M.select_from_results(conversions, {
        prompt = opts.prompt or "Convert to:"
    }, function(chosen_value, chosen_key)
        if chosen_value then
            local Core = require("alchemy.core")
            Core.replace_text(selection, chosen_value)
            
            if opts.notify then
                vim.notify(string.format("Converted to %s: %s", chosen_key or "result", chosen_value))
            end
        end
    end)
end

return M