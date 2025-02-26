require 'spec_helper'
require 'svix'
require_relative './json_responses'

describe 'API Client' do
  let!(:svx) { Svix::Client.new 'testsk_token.eu', Svix::SvixOptions.new(true, 'http://api.example.com') }
  let(:host) { 'http://api.example.com' }

  it 'token/user-agent is sent' do
    stub_request(:get, "#{host}/api/v1/app")
      .to_return(
        status: 200,
        body: ListResponseAppOut_JSON
      )
    puts svx.application.list
    expect('John Doe').to eq('John Doe')
    expect(WebMock).to have_requested(:get, "#{host}/api/v1/app")
      .with(
        headers: { 'Authorization' => 'Bearer testsk_token.eu',
                   'User-Agent' => "svix-libs/#{Svix::VERSION}/ruby" }
      )
  end
  it 'test Date in query param' do
  end
  it 'test Date request body' do
  end
  it 'test Date response body' do
  end
  it 'test listResponseOut deserializes correctly' do
  end
  it 'test enum as query param' do
  end
  it 'test list as query param' do
  end
  it 'test header param sent' do
  end
  it 'test retry for status => 500' do
  end
  it 'no body in response does not return anything' do
  end
  it '422 returns validation error' do
  end
  it '400 returns ApiException' do
  end
  it 'sub-resource works' do
  end
  it 'integer enum serialization' do
  end
  it 'string enum de/serialization' do
  end
  it 'non-camelCase field name' do
  end
  it 'patch request body' do
  end
  it 'arbitrary json object body' do
  end
end
