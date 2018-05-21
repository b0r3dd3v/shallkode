#!/usr/bin/env bats

load test_helper

run_with_git_config() {
  ROADRUNNER_PROMPT="{git:(%branch%)}" run $ROADRUNNER_BIN
}

@test "git: when not in a git repo" {
  run_with_git_config

  assert_success
  assert_output ""
}

@test "git: when in a git repo" {
  create_git_origin "repo"
  clone_origin "repo"
  cd_local "repo"

  run_with_git_config

  assert_success
  assert_output "(master)"
}
