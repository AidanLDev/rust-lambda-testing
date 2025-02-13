#!/usr/bin/env node
import * as cdk from "aws-cdk-lib";
import { DeployStack } from "../lib/deploy-stack";

const app = new cdk.App();
new DeployStack(app, "DeployStack", {
  env: {
    region: "eu-west-2",
  },
  /* If you don't specify 'env', this stack will be environment-agnostic.
   * Account/Region-dependent features and context lookups will not work,
   * but a single synthesized template can be deployed anywhere. */

  /* For more information, see https://docs.aws.amazon.com/cdk/latest/guide/environments.html */
});
