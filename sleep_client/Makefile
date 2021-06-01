version :=v1.0.0
build_tag :=kozmod/rust-sleep-client:${version}

docker:
	docker image build -t ${build_tag} . && make docker.rmi.builder

docker.rmi.builder:
	docker image prune --filter label=stage=builder <<< y