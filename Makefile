all:
	cargo build
	cp target/debug/expert_system .

clean:
	cargo clean

fclean: clean
	rm -f expert_system

re: fclean all
