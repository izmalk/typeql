/*
 * Copyright (C) 2022 Vaticle
 *
 * Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements.  See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership.  The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License.  You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */

use cucumber::{gherkin::Step, given, then, when, World};
use typeql::{parse_query, query::Query};

#[derive(Debug, Default, World)]
pub struct TypeQLWorld;

fn parse_query_in_step(step: &Step) -> Query {
    parse_query(step.docstring.as_ref().unwrap()).unwrap()
}

fn reparse_query(parsed: &Query) -> Query {
    let query_string = &parsed.to_string();
    parse_query(query_string).unwrap()
}

macro_rules! generic_step_impl {
    {$($(#[step($pattern:expr)])+ async fn $fn_name:ident $args:tt $body:tt)+} => {
        $($(
        #[given($pattern)]
        #[when($pattern)]
        #[then($pattern)]
        )*
        async fn $fn_name $args $body
        )*
    };
}

generic_step_impl! {
    #[step("reasoning schema")]
    #[step("typeql define")]
    async fn typeql_define(_: &mut TypeQLWorld, step: &Step) {
        let parsed = parse_query_in_step(step);
        assert_eq!(parsed, reparse_query(&parsed));
    }

    #[step("typeql undefine")]
    async fn typeql_undefine(_: &mut TypeQLWorld, step: &Step) {
        let parsed = parse_query_in_step(step);
        assert_eq!(parsed, reparse_query(&parsed));
    }

    #[step("get answers of typeql insert")]
    #[step("reasoning data")]
    #[step("typeql insert")]
    async fn typeql_insert(_: &mut TypeQLWorld, step: &Step) {
        let parsed = parse_query_in_step(step);
        assert_eq!(parsed, reparse_query(&parsed));
    }

    #[step("typeql delete")]
    async fn typeql_delete(_: &mut TypeQLWorld, step: &Step) {
        let parsed = parse_query_in_step(step);
        assert_eq!(parsed, reparse_query(&parsed));
    }

    #[step("typeql update")]
    async fn typeql_update(_: &mut TypeQLWorld, step: &Step) {
        let parsed = parse_query_in_step(step);
        assert_eq!(parsed, reparse_query(&parsed));
    }

    #[step("get answer of typeql get aggregate")]
    #[step("get answers of typeql get group aggregate")]
    #[step("get answers of typeql get group")]
    #[step("get answers of typeql get")]
    #[step("get answers of typeql fetch")]
    #[step("reasoning query")]
    #[step("verify answer set is equivalent for query")]
    async fn typeql_get(_: &mut TypeQLWorld, step: &Step) {
        let parsed = parse_query_in_step(step);
        assert_eq!(parsed, reparse_query(&parsed));
    }

    #[step("aggregate answer is empty")]
    #[step("group aggregate answer value is empty")]
    #[step("answer groups are")]
    #[step("answers contain explanation tree")]
    #[step("fetch answers are")]
    #[step("concept identifiers are")]
    #[step("connection close all sessions")]
    #[step("connection does not have any database")]
    #[step("connection opens with default authentication")]
    #[step("connection has been opened")]
    #[step("each answer satisfies")]
    #[step("group aggregate values are")]
    #[step("order of answer concepts is")]
    #[step("rules are")]
    #[step("session opens transaction of type: read")]
    #[step("session opens transaction of type: write")]
    #[step("session transaction closes")]
    #[step("session transaction is open: false")]
    #[step("for each session, transaction closes")]
    #[step("templated typeql get; throws exception")]
    #[step("transaction commits")]
    #[step("transaction commits; throws exception")]
    #[step("transaction is initialised")]
    #[step("typedb starts")]
    #[step("uniquely identify answer concepts")]
    #[step("verifier is initialised")]
    #[step("verify answers are complete")]
    #[step("verify answers are sound")]
    #[step(regex = "connection create database: .*")]
    #[step(regex = r"^aggregate value is: .*$")]
    #[step(regex = r"^answer size is: .*$")]
    #[step(regex = r"^connection open data session for database: .*$")]
    #[step(regex = r"^connection open schema session for database: .*$")]
    #[step(regex = r"^rules contain: .*$")]
    #[step(regex = r"^rules do not contain: .*$")]
    #[step(regex = r"^set time-zone is: .*$")]
    #[step(regex = r"^verify answer size is: .*$")]
    #[step(regex = r"^verify answers are consistent across .* executions$")]
    async fn do_nothing_no_connection(_: &mut TypeQLWorld) {}

    #[step("typeql define; throws exception")]
    #[step("typeql delete; throws exception")]
    #[step("typeql insert; throws exception")]
    #[step("typeql get aggregate; throws exception")]
    #[step("typeql get group; throws exception")]
    #[step("typeql get; throws exception")]
    #[step("typeql fetch; throws exception")]
    #[step("typeql undefine; throws exception")]
    #[step("typeql update; throws exception")]
    #[step(regex = r"^typeql get; throws exception containing .*$")]
    async fn do_nothing_unknown_exception(_: &mut TypeQLWorld) {}
}
