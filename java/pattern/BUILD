#
# Copyright (C) 2022 Vaticle
#
# Licensed to the Apache Software Foundation (ASF) under one
# or more contributor license agreements.  See the NOTICE file
# distributed with this work for additional information
# regarding copyright ownership.  The ASF licenses this file
# to you under the Apache License, Version 2.0 (the
# "License"); you may not use this file except in compliance
# with the License.  You may obtain a copy of the License at
#
#   http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing,
# software distributed under the License is distributed on an
# "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
# KIND, either express or implied.  See the License for the
# specific language governing permissions and limitations
# under the License.
#

package(default_visibility = ["//visibility:public"])

load("@vaticle_bazel_distribution//maven:rules.bzl", "assemble_maven", "deploy_maven")
load("@vaticle_dependencies//tool/checkstyle:rules.bzl", "checkstyle_test")
load("@vaticle_dependencies//distribution:deployment.bzl", "deployment")

java_library(
    name = "pattern",
    srcs = glob([
        "*.java",
        "constraint/*.java",
        "expression/*.java",
        "schema/*.java",
        "statement/*.java",
        "statement/builder/*.java",
        "util/*.java",
    ]),
    deps = [
        # Internal Package Dependencies
        "//java/common:common",

        # Internal Repository Dependencies
        "@vaticle_typedb_common//:common",

        # External dependencies
        "@maven//:com_google_code_findbugs_jsr305",
    ],
    tags = ["maven_coordinates=com.vaticle.typeql:typeql-pattern:{pom_version}"],
)

assemble_maven(
  name = "assemble-maven",
  target = ":pattern",
  workspace_refs = "@vaticle_typeql_workspace_refs//:refs.json"
)

deploy_maven(
    name = "deploy-maven",
    target = ":assemble-maven",
    snapshot = deployment['maven.snapshot'],
    release = deployment['maven.release']
)

checkstyle_test(
    name = "checkstyle",
    include = glob([
        "*",
        "constraint/*.java",
        "expression/*.java",
        "schema/*.java",
        "statement/*.java",
        "statement/builder/*.java",
        "util/*.java",
    ]),
    license_type = "apache-header",
)
