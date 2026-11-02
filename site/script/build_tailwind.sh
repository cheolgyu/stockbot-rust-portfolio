echo "hello  ./script/build_tailwind.sh ";
echo "Username: $1";

echo "mode = $1"
echo $PWD
echo " ================================================== "
if [ mode = "debug" ]
then
    echo "mode is debug "
    # npx tailwindcss -i  src/css/tailwind.css -o ./tailwind.css
    npx @tailwindcss/cli -i ./src/css/input_tailwind.css -o ./output_tailwind.css --watch
else
    echo "mode is not debug "
    npx @tailwindcss/cli -i ./src/css/input_tailwind.css -o ./output_tailwind.css --minify
fi
echo " ================================================== "