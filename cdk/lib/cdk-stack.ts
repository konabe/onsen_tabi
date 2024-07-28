import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import * as ec2 from "aws-cdk-lib/aws-ec2";
import * as iam from "aws-cdk-lib/aws-iam";
import * as elbv2 from "aws-cdk-lib/aws-elasticloadbalancingv2";
import * as autoscaling from "aws-cdk-lib/aws-autoscaling";
import { Certificate } from "aws-cdk-lib/aws-certificatemanager";

const createName = (text: string) => `onsen-tabi-${text}`;

export class CdkStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);
    // TODO: VPCをここで定義する
    const vpc = ec2.Vpc.fromLookup(this, "defaultVpc", {
      vpcId: process.env.CDK_VPC_ID!,
    });

    const securityGroupALB = new ec2.SecurityGroup(this, "SecurityGroupALB", {
      securityGroupName: createName("alb-sg"),
      vpc,
      description: "for onsen-tabis alb",
    });
    securityGroupALB.addIngressRule(ec2.Peer.anyIpv4(), ec2.Port.HTTPS);
    securityGroupALB.applyRemovalPolicy(cdk.RemovalPolicy.DESTROY);

    const securityGroupEC2 = new ec2.SecurityGroup(this, "SecurityGroupEC2", {
      securityGroupName: createName("web-server-sg"),
      vpc,
      description: "for onsen-tabis web server",
    });
    securityGroupEC2.addIngressRule(securityGroupALB, ec2.Port.HTTP);
    securityGroupEC2.addIngressRule(ec2.Peer.anyIpv4(), ec2.Port.SSH);
    securityGroupEC2.addIngressRule(
      ec2.Peer.anyIpv4(),
      ec2.Port.HTTP,
      "for debug"
    );
    securityGroupEC2.applyRemovalPolicy(cdk.RemovalPolicy.DESTROY);

    const keyPair = new ec2.KeyPair(this, "KeyPair", {
      keyPairName: createName("key-pair"),
    });
    keyPair.applyRemovalPolicy(cdk.RemovalPolicy.DESTROY);

    // TODO: IAMRoleをここで定義する
    const ec2InstanceRole = iam.Role.fromRoleArn(
      this,
      "InstanceRole",
      process.env.CDK_EC2_INSTANCE_ROLE_ARN!,
      {
        mutable: false,
      }
    );

    const dockerHubRepository = process.env.CDK_DOCKER_HUB_REPOSITORY!;
    const launchTemplate = new ec2.LaunchTemplate(this, "WebServerTemplate", {
      launchTemplateName: createName("web-server-tpl"),
      associatePublicIpAddress: true,
      instanceMetadataTags: true,
      machineImage: new ec2.GenericLinuxImage({
        "ap-northeast-1": "ami-034c9ca2bdde7b472",
      }),
      instanceType: ec2.InstanceType.of(
        ec2.InstanceClass.T2,
        ec2.InstanceSize.MICRO
      ),
      keyPair,
      // subnet
      securityGroup: securityGroupEC2,
      role: ec2InstanceRole,
      userData: ec2.UserData.custom(
        `#cloud-config
        timezone: Asia/Tokyo
        locale: ja_JP.utf8
        runcmd:
          - sudo yum install -y docker
          - sudo service docker start
          - sudo usermod -a -G docker ec2-user
          - sudo docker pull ${dockerHubRepository}:latest
          - export DATABASE_URL=\`aws ssm get-parameter --with-decryption --name /prod/DATABASE_URL | jq -r ".Parameter.Value"\`
          - export JWT_SECRET_KEY=\`aws ssm get-parameter --with-decryption --name /prod/JWT_SECRET_KEY | jq -r ".Parameter.Value"\`
          - sudo docker run --rm --env DATABASE_URL="$DATABASE_URL" --env JWT_SECRET_KEY="$JWT_SECRET_KEY" --publish 8000:8000 --name web_server ${dockerHubRepository}:latest
        `
      ),
    });
    launchTemplate.applyRemovalPolicy(cdk.RemovalPolicy.DESTROY);
    cdk.Tags.of(launchTemplate).add(
      "Name",
      createName("web-server-ec2-instance")
    );

    const certificate = Certificate.fromCertificateArn(
      this,
      "Certificate",
      process.env.CDK_CERTIFICATE_ARN!
    );

    const alb = new elbv2.ApplicationLoadBalancer(this, "ALB", {
      loadBalancerName: createName("alb"),
      vpc,
      internetFacing: true,
      securityGroup: securityGroupALB,
    });
    alb.applyRemovalPolicy(cdk.RemovalPolicy.DESTROY);

    const targetGroup = new elbv2.ApplicationTargetGroup(this, "TargetGroup", {
      targetGroupName: createName("target-group"),
      healthCheck: {
        path: "/",
        protocol: elbv2.Protocol.HTTP,
        healthyHttpCodes: "200",
        healthyThresholdCount: 5,
        unhealthyThresholdCount: 2,
        timeout: cdk.Duration.seconds(5),
        interval: cdk.Duration.seconds(30),
      },
      port: 8000,
      protocol: elbv2.ApplicationProtocol.HTTP,
      vpc,
    });

    const asg = new autoscaling.AutoScalingGroup(this, "Auto Scaling Group", {
      autoScalingGroupName: createName("asg"),
      vpc,
      launchTemplate,
    });
    asg.attachToApplicationTargetGroup(targetGroup);
    asg.applyRemovalPolicy(cdk.RemovalPolicy.DESTROY);

    const listener = alb.addListener("Listener", { port: 443 });
    listener.addCertificates("Certificate", [certificate]);
    listener.addTargetGroups("TargetGroup", {
      targetGroups: [targetGroup],
    });

    // TODO: Route53から関連付ける。
  }
}
