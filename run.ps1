$name = $args[0]

cd .\$name
if (!$?) { exit }

rustc .\$name.rs
if (!$?) { cd ..; exit }

& .\$name.exe
if (!$?) { cd ..; exit }

cd ..
