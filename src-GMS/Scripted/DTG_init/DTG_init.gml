/// @function							DTG_init( dll_name)
/// @param	{string}	[dll_name]		Name of the DLL file (also can be a path)
function DTG_init(){
	if variable_global_exists("DTG_GetTokens") exit;
	globalvar DTG_GetTokens;
	var _dll_name;
	if argument_count > 0 {
		_dll_name = argument[0];
	} else {
		_dll_name = "dtg-100.dll";
	}
	DTG_GetTokens = external_define(_dll_name, "GetTokens", dll_cdecl, ty_string, 1, ty_string);
}