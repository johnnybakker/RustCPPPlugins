#include <iostream>
#include <my_devkit_plugin.hpp>
#include <boost/dll.hpp>
#include <boost/function.hpp>
#include <boost/filesystem.hpp>
#include <boost/filesystem/operations.hpp>

using namespace std;
using namespace boost::filesystem;
using namespace boost::dll;


int main(int argc, char* argv) {

	auto program_dir = program_location().parent_path();
	auto plugins_dir = program_dir / "plugins";

	for (directory_iterator it (plugins_dir); it != directory_iterator(); ++it)
	{
		if(!it->is_regular_file()) continue;
        
		
		library_info inf(it->path());
		shared_library library(it->path());
	
		for(const string symbol_name : inf.symbols(".plugins")) {
			//auto symbol = library.get(symbol_name);
			cout << "Found plugin symbol: " << symbol_name << endl;
		}

		
	}

	return 0;
}	