#/bin/bash

cargo build
cp target/debug/expert_system .
for FILE in $(ls test); do
	echo "---------------"
	cat test/$FILE
	echo "---------------"
	./expert_system test/$FILE
done
