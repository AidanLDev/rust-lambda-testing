import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import {
  Code,
  Function,
  Runtime,
  FunctionUrlAuthType,
} from "aws-cdk-lib/aws-lambda";
import * as iam from "aws-cdk-lib/aws-iam";
import { CfnOutput } from "aws-cdk-lib";
import path = require("path");

export class DeployStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const handler = new Function(this, "MyFunction", {
      code: Code.fromAsset(
        path.join(__dirname, "..", "..", "target/lambda/rust-lambda-testing"),
      ),
      runtime: Runtime.PROVIDED_AL2023,
      handler: "does_not_matter",
      functionName: "rust-lambda-testing",
    });

    handler.addToRolePolicy(
      new iam.PolicyStatement({
        actions: ["dynamodb:ListTables"],
        resources: ["*"],
      }),
    );

    const fnUrl = handler.addFunctionUrl({
      authType: FunctionUrlAuthType.NONE,
    });

    new CfnOutput(this, "TheUrl", {
      value: fnUrl.url,
    });
  }
}
