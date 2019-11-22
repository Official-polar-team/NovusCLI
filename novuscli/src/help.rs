use std::process::Command;
use std::process;
use std::env;

pub fn help_menu() {
	//Prints help menu.
		//SaturnCLI Help
		println!("\n[1mSaturnCLI Help[0m\n");
		//stn search <query>
		println!("[1;35mstn [0m[1;33msearch <query>[0m\t\t\tSearches for specified query");
		//stn list [--flag(s)] <package(s)>
		println!("[1;35mstn [0m[1;33mlist [--flag(s)] <package(s)>[0m\tLists specified packages");
		//stn info <package(s)>
		println!("[1;35mstn [0m[1;33minfo <package(s)>[0m\t\t\tDisplay info on specified package(s)");
		//stn install <package(s)>
		println!("[1;35mstn [0m[1;33minstall <package(s)>[0m\t\tInstalls specified package(s)");
		//stn reinstall <package(s)>
		println!("[1;35mstn [0m[1;33mreinstall <package(s)>[0m\t\tReinstalls specified package(s)");
		//stn remove <package(s)>
		println!("[1;35mstn [0m[1;33mremove <package(s)>[0m\t\t\tRemoves specified package(s)");
		//stn add-key <filepath>
		println!("[1;35mstn [0m[1;33madd-key <filepath>[0m\t\t\tAdds a key to the list of trusted keys");
		//stn edit-sources
		println!("[1;35mstn [0m[1;33medit-sources[0m\t\t\tOpens the APT repo editor");
		//stn add-repositories [--flag(s)] <repo(s)>
		println!("[1;35mstn [0m[1;33madd-repos [--flag(s)] <repo(s)>[0m\tAdds new repo(s)");
		//stn autoremove
		println!("[1;35mstn [0m[1;33mautoremove[0m\t\t\t\tRemoves unneeded packages (orphans)");
		//stn update
		println!("[1;35mstn [0m[1;33mupdate[0m\t\t\t\tUpdates the repository lists");
		//stn upgrade <package(s)>
		println!("[1;35mstn [0m[1;33mupgrade <package(s)>[0m\t\tUpgrades specified packages");
		if cfg!(target_os = "windows") || cfg!(target_os = "linux") {
			//stn full-upgrade <package(s)>
			println!("[1;35mstn [0m[1;33mfull-upgrade <package(s)>[0m\t\tUpgrades the system (or package(s))");
		}
		//stn version
		println!("[1;35mstn [0m[1;33mversion[0m\t\t\t\tShow APT, DPKG, and SaturnCLI versions");
		//stn clean
		println!("[1;35mstn [0m[1;33mclean[0m\t\t\t\tClears the download cache");
		//stn help <command>
		println!("[1;35mstn [0m[1;33mhelp <command>[0m\t\t\tOpens help menu for specified commands");
		//stn about
		println!("[1;35mstn [0m[1;33mabout[0m\t\t\t\tView legal information and credits\n");
		process::exit(0);
}