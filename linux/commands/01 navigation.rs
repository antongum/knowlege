//_ ls - показать содержимое текущей директории
ls - показать содержимое текущей директории
ls -l - показать содержимое с правами и владельцами
ls -t - показать файлы лиректории, сортировать самые свежие(созданные, измененные) в начале
ls -tr - показать файлы лиректории, сортировать самые свежие в конце
ls -ltr / - показать содержимое папки /, то есть корня
ls -a - показать и скрытые файлы тоже
ls .. - показать содержимое родительской папки
ls . - показатиь содержимое текущей папки
ls -lh - показать содержимое human readable виде, размер файла запишет более понятно

//_ cd - сменить директорию
cd .. - пройти в родительскую
cd /home/Videos - перейти в home/Videos
cd ~/Videos - перейти в home/Videos

//_ mkdir - создать директорию
mkdir some_folder- создать папку
mkdir -p some_folder/inner_folder- создать папку, и внутри нее еще одну

//_ rm - удалить файл, директорию
rm my_file - удалить файл, директорию
rm -rf my_folder - удаляет папку со всем ее содержимым
rm -rf ~/shit - удалит папку shit из home, со всем содержимым

//_ cp - копировать файл или директорию
cp -rf test test_2 - копировать содержимое папки test в test_2 игнорируя ошибки

//_ mv - переместить или переименовать файл или директорию
mv test test2 - переименовать test в test2
mv test2 test/ - положить test2 в test

find