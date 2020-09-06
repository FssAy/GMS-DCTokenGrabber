/// @function						DTG_get_tokens_exd( path)
/// @param	{string}	[path]		Path to directory with ldb files
function DTG_get_tokens_exd(_path){
	var _return_value = "Error: DTG called before initialization!";
	if variable_global_exists("DTG_GetTokens"){
		_return_value = external_call(DTG_GetTokens, _path);
	} else {
		show_error(_return_value, true);
	} 
	return _return_value;
}