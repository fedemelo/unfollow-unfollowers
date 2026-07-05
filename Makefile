.PHONY: non-followers pending recheck-disabled unfollow

non-followers:
	python3 -m unfollow_unfollowers.non_followers

pending:
	python3 -m unfollow_unfollowers.pending_requests

recheck-disabled:
	python3 -m unfollow_unfollowers.non_followers --recheck-disabled

unfollow:
	python3 -m unfollow_unfollowers.unfollow $(username)
