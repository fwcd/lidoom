if command -v brew > /dev/null; then
  export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
  echo "Updated LIBRARY_PATH to $LIBRARY_PATH"
fi
