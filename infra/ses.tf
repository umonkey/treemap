resource "aws_ses_domain_identity" "main" {
  domain = var.domain
}

resource "aws_ses_domain_dkim" "main" {
  domain = aws_ses_domain_identity.main.domain
}

data "cloudflare_zone" "main" {
  name = var.domain
}

resource "cloudflare_record" "ses_verification" {
  zone_id = data.cloudflare_zone.main.id
  name    = "_amazonses"
  type    = "TXT"
  content = "\"${aws_ses_domain_identity.main.verification_token}\""
  ttl     = 600
}

resource "cloudflare_record" "ses_dkim" {
  count   = 3
  zone_id = data.cloudflare_zone.main.id
  name    = "${aws_ses_domain_dkim.main.dkim_tokens[count.index]}._domainkey"
  type    = "CNAME"
  content = "${aws_ses_domain_dkim.main.dkim_tokens[count.index]}.dkim.amazonses.com"
  ttl     = 600
}

resource "cloudflare_record" "spf" {
  zone_id = data.cloudflare_zone.main.id
  name    = "@"
  type    = "TXT"
  content = "\"v=spf1 include:amazonses.com include:zoho.eu ~all\""
  ttl     = 3600
}

resource "cloudflare_record" "dmarc" {
  zone_id = data.cloudflare_zone.main.id
  name    = "_dmarc"
  type    = "TXT"
  content = "\"v=DMARC1; p=none;\""
  ttl     = 3600
}

resource "aws_iam_user" "ses_smtp_user" {
  name = "ses-smtp-user"
}

resource "aws_iam_access_key" "ses_smtp_user" {
  user = aws_iam_user.ses_smtp_user.name
}

resource "aws_iam_user_policy" "ses_smtp_policy" {
  name = "ses-smtp-policy"
  user = aws_iam_user.ses_smtp_user.name

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect   = "Allow"
        Action   = "ses:SendRawEmail"
        Resource = "*"
      }
    ]
  })
}
