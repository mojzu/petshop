{{/*
Expand the name of the chart.
*/}}
{{- define "petshop.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create a default fully qualified app name.
We truncate at 63 chars because some Kubernetes name fields are limited to this (by the DNS naming spec).
If release name contains chart name it will be used as a full name.
*/}}
{{- define "petshop.fullname" -}}
{{- if .Values.fullnameOverride }}
{{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- $name := default .Chart.Name .Values.nameOverride }}
{{- if contains $name .Release.Name }}
{{- .Release.Name | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- printf "%s-%s" .Release.Name $name | trunc 63 | trimSuffix "-" }}
{{- end }}
{{- end }}
{{- end }}

{{- define "petshop.serverName" -}}
{{- printf "%s-%s" (include "petshop.fullname" .) "server" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{- define "petshop.serverExampleJobName" -}}
{{- printf "%s-%s" (include "petshop.fullname" .) "example-job" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{- define "petshop.envoyName" -}}
{{- printf "%s-%s" (include "petshop.fullname" .) "envoy" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create chart name and version as used by the chart label.
*/}}
{{- define "petshop.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Common labels
*/}}
{{- define "petshop.labels" -}}
helm.sh/chart: {{ include "petshop.chart" . }}
{{ include "petshop.selectorLabels" . }}
{{- if .Chart.AppVersion }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
{{- end }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end }}

{{/*
Selector labels
*/}}
{{- define "petshop.selectorLabels" -}}
app.kubernetes.io/name: {{ include "petshop.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}

{{- define "petshop.serverSelectorLabels" -}}
{{ include "petshop.selectorLabels" . }}
app.kubernetes.io/selector: {{ include "petshop.serverName" . }}
{{- end }}

{{- define "petshop.envoySelectorLabels" -}}
{{ include "petshop.selectorLabels" . }}
app.kubernetes.io/selector: {{ include "petshop.envoyName" . }}
{{- end }}

{{/*
Create the name of the service account to use
*/}}
{{- define "petshop.serviceAccountName" -}}
{{- if .Values.serviceAccount.create }}
{{- default (include "petshop.fullname" .) .Values.serviceAccount.name }}
{{- else }}
{{- default "default" .Values.serviceAccount.name }}
{{- end }}
{{- end }}
