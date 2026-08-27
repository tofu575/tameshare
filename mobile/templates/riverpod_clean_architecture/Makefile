PACKAGE_DIRS = \
	lib/domain/model \
	lib/domain/usecase \
	lib/gateway/dummy_gateway \
	lib/presentation

.PHONY: pub-get format analyze test

pub-get:
	flutter pub get
	@for package_dir in $(PACKAGE_DIRS); do \
		(cd $$package_dir && flutter pub get) || exit 1; \
	done

format:
	dart format lib test

analyze:
	flutter analyze

test:
	flutter test
