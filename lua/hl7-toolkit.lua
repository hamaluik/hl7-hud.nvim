local augroup = vim.api.nvim_create_augroup("hl7-toolkit", { clear = true })

local function setup()
    -- add plugin path to runtimepath
    local plugin_path = debug.getinfo(1, 'S').source:sub(2):match("(.*/)")
    plugin_path = plugin_path:sub(1, -6)
    vim.opt.rtp:append(plugin_path)

    -- detect hl7 file types
    vim.api.nvim_create_autocmd({"BufRead", "BufNewFile"},
        { group = augroup, desc = "Set filetype to hl7", pattern = "*.hl7", callback = function()
            vim.bo.filetype = "hl7"
        end })
end

local function build()
    local plugin_path = debug.getinfo(1, 'S').source:sub(2):match("(.*/)")
    plugin_path = plugin_path:sub(1, -6)
    
    local result = vim.api.system({"cargo", "build", "--release"}, {
        cwd = plugin_path
    
    }):wait()
    
    if result.code == 0 then
        print("hl7-tools cargo build successful")
    else
        print("hl7-tools cargo build failed: " .. result.stderr)
    end

    local result = vim.api.system({"ln", "-s", "target/release/libhl7_tools.so", "lua/hl7_tools.so"}, {
        cwd = plugin_path
    }):wait()

    if result.code == 0 then
        print("hl7-tools link successful")
    else
        print("hl7-tools link failed: " .. result.stderr)
    end
end

local function locate_cursor()
    return require('hl7_tools').locate_cursor()
end

local function print_cursor_pos()
    print(require('hl7_tools').locate_cursor())
end

local function query_input()
    local q = vim.fn.input("HL7 Query: ")
    if q ~= "" then
        require('hl7_tools').goto_query(q)
    end
end

local function parse_timestamp()
    return require('hl7_tools').parse_timestamp_at_cursor()
end

local function print_timestamp()
    print(require('hl7_tools').parse_timestamp_at_cursor())
end

local function generate_timestamp_utc()
    return require('hl7_tools').generate_timestamp(true)
end

local function generate_timestamp_local()
    return require('hl7_tools').generate_timestamp(true)
end

local function insert_timestamp(utc)
    local ts = utc and generate_timestamp_utc() or generate_timestamp_local()
    vim.api.nvim_put({ts}, "c", true, true)
end

local function insert_control_id()
    local cid = require('hl7_tools').generate_control_id()
    vim.api.nvim_put({cid}, "c", true, true)
end

local LLEXT = {}
LLEXT.sections = {
    lualine_a = {'mode'},
    lualine_b = {'branch', 'diff', 'diagnostics'},
    lualine_c = {'filename'},
    lualine_x = {'encoding', 'fileformat', 'filetype'},
    lualine_y = {'progress'},
    lualine_z = {'location', locate_cursor},
}
LLEXT.filetypes = { "hl7" }

return {
    setup = setup,
    build = build,
    locate_cursor = locate_cursor,
    print_cursor_pos = print_cursor_pos,
    query_input = query_input,
    parse_timestamp = parse_timestamp,
    print_timestamp = print_timestamp,
    generate_timestamp_utc = generate_timestamp_utc,
    generate_timestamp_local = generate_timestamp_local,
    insert_timestamp = insert_timestamp,
    insert_control_id = insert_control_id,
    lualine_ext = LLEXT,
}
