$project = ($args[0]).Replace('.\', '').Replace('\','')
$src_list = @($project, 'main')

$dir = (Get-Location)
$src = ''
$bin = ''
$bin_args = ($args | Select-Object -Skip 1)

try {
	if (!(Test-Path -Path .\$project)) {
		Write-Host "Can't find project folder: .\$project"
		exit
	}
	cd .\$project
	
	foreach ($s in $src_list) {
		if (Test-Path -Path .\$s.rs) {
			$src = "$s.rs"
			$bin = "$s.exe"
			break
		}
	}

	if ($src -like "*.rs") {
		rustc .\$src
		if ($?) {
			& .\$bin $bin_args
		}
	} else {
		Write-Host "Can't find source file."
	}
}
finally {
	cd $dir
}
