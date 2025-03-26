GRAPH_DIR=benchmark_graphs

all: $(GRAPH_DIR)/combined_benchmark.png

$(GRAPH_DIR)/results.txt:
	mkdir -p $(GRAPH_DIR)
	cargo bench > $@

$(GRAPH_DIR)/combined_benchmark.png: $(GRAPH_DIR)/results.txt
	python3 bench-graph.py --input $<

clean:
	rm -rf $(GRAPH_DIR)/*.png $(GRAPH_DIR)/*.txt