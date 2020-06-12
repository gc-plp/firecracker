for i in $(find ./tests/integration_tests/* -name "*.py"); do
    echo "- command: ./tools/devtool -y test -- .$i"
done
