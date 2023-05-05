import * as path from 'path';
import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import { RustFunction } from 'cargo-lambda-cdk';
import {HttpApi, HttpMethod} from '@aws-cdk/aws-apigatewayv2-alpha';
import {HttpLambdaIntegration} from '@aws-cdk/aws-apigatewayv2-integrations-alpha';

export class DeployStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    // Create the upload function
    const uploadFunction = new RustFunction(this, 'markthis', {
        binaryName: 'upload',
        manifestPath: path.join(__dirname, '..', '..', 'Cargo.toml'),
    });

    // Create the API
    const api = new HttpApi(this, 'markthis-api');

    // Add the upload function as an integration
    api.addRoutes({
        path: '/upload',
        methods: [HttpMethod.POST],
        integration: new HttpLambdaIntegration(
            'upload-integration',
            uploadFunction,
        ),
    });

    // Grant the upload function access to the API
    uploadFunction.addEnvironment('API_URL', api.url!);

    // Add the S3 bucket
    const bucket = new cdk.aws_s3.Bucket(this, 'markthis-upload-bucket');

    // Grant the upload function ability to generate pre-signed URLs
    bucket.grantPut(uploadFunction);

    // Add the bucket name to the upload function's environment
    uploadFunction.addEnvironment('BUCKET_NAME', bucket.bucketName);

    // Output the API URL
    new cdk.CfnOutput(this, 'api-url', {
        value: api.url!,
    });

    // Output the upload function name
    new cdk.CfnOutput(this, 'upload-function-name', {
        value: uploadFunction.functionName,
    });

    // Output the upload function ARN
    new cdk.CfnOutput(this, 'upload-function-arn', {
        value: uploadFunction.functionArn,
    });
  }
}