.PHONY: non-followers recheck-disabled

non-followers:
	python3 -m unfollow_unfollowers.non_followers

recheck-disabled:
	python3 -m unfollow_unfollowers.non_followers --recheck-disabled
